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
    let channel_0 = connection.create_channel().await?;
    let channel_1 = connection.create_channel().await?;
    let () = channel_1.close(200 as u16, "CIAO!").await?;
    let () = channel_0.close(200 as u16, "CIAO!").await?;
    let () = connection.close(200 as u16, "See you!").await?;

    Ok(())
}
