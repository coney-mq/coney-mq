use ::futures::prelude::*;

use ::examples::*;

use ::lapin::options::{
    BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
};
use ::lapin::types::FieldTable;
use ::lapin::{Connection, ExchangeKind};

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
            "e_10_1",
            exchange_kind.clone(),
            exchange_declare_opts.clone(),
            exchange_declare_args.clone(),
        )
        .await?;

    let queue = channel_0
        .queue_declare("q_10_1", queue_declare_opts.clone(), queue_declare_args.clone())
        .await?;

    let () = channel_0
        .queue_bind("q_10_1", "e_10_1", "#", queue_bind_opts.clone(), queue_bind_args.clone())
        .await?;

    let mut consumer = channel_0
        .basic_consume(
            queue.name().as_str(),
            "consume-q_10_1",
            Default::default(),
            Default::default(),
        )
        .await?;

    println!("Consumer: {:#?}", consumer);

    let basic_publish_opts = BasicPublishOptions { mandatory: true, immediate: true };

    let confirm = channel_0
        .basic_publish(
            "e_10_1",
            "please-pretty-please",
            basic_publish_opts,
            "Well, hello!".as_bytes(),
            Default::default(),
        )
        .await?;

    let confirmed = confirm.await?;

    println!("Confirmed: {:#?}", confirmed);

    let delivery = consumer.next().await.expect("No delivery! Why?")?;
    println!("Delivery: {:#?}", delivery);

    let () = connection.close(200 as u16, "See you!").await?;

    Ok(())
}
