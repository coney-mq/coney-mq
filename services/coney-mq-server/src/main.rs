use std::net::SocketAddr;
use std::sync::Arc;

use ::amqp_0_9_1::backend::Backend;
use ::amqp_0_9_1::config::AmqpConfig;
use ::amqp_0_9_1::config::ConnectionLimits;
use ::authc::Authc;
use ::authc::AuthcWithMechs;
use ::common::ErrorReport;

use ::futures::prelude::*;

use ::amqp_0_9_1::amqp_framing::AmqpFraming;
use ::amqp_0_9_1::connection::AmqpConnection;
use ::amqp_0_9_1::listener::AmqpListener;
use ::common::AnyError;

#[tokio::main]
async fn main() {
    if let Err(failure) = run().await {
        eprintln!("Application Failure:\n{}", failure.error_report());
    }
}

async fn run() -> Result<(), AnyError> {
    let _ = dotenv::dotenv();
    let () = ::pretty_env_logger::init_timed();

    let bind_addr: SocketAddr = "0.0.0.0:5672".parse().unwrap();

    let (tcp_tx, mut tcp_rx) = ::futures::channel::mpsc::unbounded();
    let tcp_listener = AmqpListener::bind_tcp(bind_addr, tcp_tx).await?;
    log::info!("Bound {:?}", bind_addr);

    // let (uds_tx, _uds_rx) = ::futures::channel::mpsc::unbounded();
    // let _uds_listener =
    //     ::amqp_0_9_1::listener::AmqpListener::bind_uds("./amqp.sock", uds_tx).await?;

    let tcp_listener_running = tcp_listener.run().map_err(AnyError::from);

    let backend = {
        let authc = AuthcWithMechs::create().with_mech(
            ::authc_plain_const_creds::AuthcMechPlainConstCreds::new(vec![
                ("guest", "guest", "guest"),
                ("admin", "admin", "admin"),
            ]),
        );
        Arc::new(BE { authc })
    };

    let tcp_inbound_spawned = async move {
        while let Some(io_stream) = tcp_rx.next().await {
            let framing = AmqpFraming::new(io_stream);
            let conn = AmqpConnection::new(framing, backend.clone());
            let conn_running = conn
                .run()
                .map_err(|conn_err| log::error!("Connection Error:\n{}", conn_err.error_report()));
            ::tokio::spawn(conn_running);
        }
        Result::<(), AnyError>::Ok(())
    };

    let _ = future::try_join(tcp_listener_running, tcp_inbound_spawned).await?;

    Ok(())
}

struct BE {
    authc: AuthcWithMechs,
}
impl AmqpConfig for BE {
    fn connection_limits(&self) -> &dyn ConnectionLimits {
        self
    }
}
impl ConnectionLimits for BE {
    fn max_channels(&self) -> u16 {
        512
    }
    fn max_frame_size(&self) -> u32 {
        10240
    }
    fn max_heartbeat(&self) -> u16 {
        300
    }
}
impl Backend for BE {
    fn amqp_config(&self) -> &dyn AmqpConfig {
        self
    }
    fn authc(&self) -> &dyn Authc {
        &self.authc
    }
}
