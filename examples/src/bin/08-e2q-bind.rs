#[macro_use]
extern crate anyhow;

use ::examples::*;

use ::lapin::options::ExchangeDeclareOptions;
use ::lapin::options::QueueBindOptions;
use ::lapin::options::QueueDeclareOptions;
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

    let exchange_kind = ExchangeKind::Topic;

    let exchange_declare_opts = ExchangeDeclareOptions::default();
    let exchange_declare_args = FieldTable::default();

    let queue_declare_opts = QueueDeclareOptions::default();
    let queue_declare_args = FieldTable::default();

    let queue_bind_opts = QueueBindOptions::default();
    let queue_bind_args = FieldTable::default();

    let () = channel_0
        .exchange_declare(
            "e_08_src",
            exchange_kind.clone(),
            exchange_declare_opts.clone(),
            exchange_declare_args.clone(),
        )
        .await?;
    let _queue = channel_0
        .queue_declare(
            "q_08_dst",
            queue_declare_opts.clone(),
            queue_declare_args.clone(),
        )
        .await?;

    let () = channel_0
        .queue_bind(
            "q_08_dst",
            "e_08_src",
            "routing-key",
            queue_bind_opts,
            queue_bind_args,
        )
        .await?;

    let () = connection.close(200 as u16, "See you!").await?;

    Ok(())
}
