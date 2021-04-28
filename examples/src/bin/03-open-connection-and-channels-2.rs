use ::examples::*;

use ::lapin::Connection;

#[tokio::main]
async fn main() {
    ::pretty_env_logger::init_timed();

    if let Err(reason) = run().await {
        log::error!("Crash");
        for cause in reason.chain() {
            log::error!("- {}", cause);
        }
    }
}

async fn run() -> Result<(), ::anyhow::Error> {
    let amqp_uri = config::amqp_uri();

    let connection = Connection::connect(&amqp_uri, Default::default()).await?;
    let _channel_0 = connection.create_channel().await?;
    let _channel_1 = connection.create_channel().await?;
    let () = connection.close(200 as u16, "See you!").await?;

    Ok(())
}
