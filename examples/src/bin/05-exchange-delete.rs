#[macro_use]
extern crate anyhow;

use ::examples::*;

use ::lapin::options::ExchangeDeclareOptions;
use ::lapin::types::FieldTable;
use ::lapin::Connection;
use ::lapin::ExchangeKind;

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

    let () = channel_0
        .exchange_delete("e_04_1", Default::default())
        .await?;
    let () = channel_0
        .exchange_delete("e_04_2", Default::default())
        .await?;

    let () = connection.close(200 as u16, "See you!").await?;

    Ok(())
}
