use ::examples::*;

use ::lapin::options::ExchangeBindOptions;
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

    let exchange_kind = ExchangeKind::Topic;

    let exchange_declare_opts = ExchangeDeclareOptions::default();
    let exchange_declare_args = FieldTable::default();

    let exchange_bind_opts = ExchangeBindOptions::default();
    let exchange_bind_args = FieldTable::default();

    let () = channel_0
        .exchange_declare(
            "e_07_src",
            exchange_kind.clone(),
            exchange_declare_opts.clone(),
            exchange_declare_args.clone(),
        )
        .await?;
    let () = channel_0
        .exchange_declare(
            "e_07_dst",
            exchange_kind.clone(),
            exchange_declare_opts.clone(),
            exchange_declare_args.clone(),
        )
        .await?;

    let () = channel_0
        .exchange_bind(
            "e_07_dst",
            "e_07_src",
            "routing-key",
            exchange_bind_opts,
            exchange_bind_args,
        )
        .await?;

    let () = connection.close(200 as u16, "See you!").await?;

    Ok(())
}
