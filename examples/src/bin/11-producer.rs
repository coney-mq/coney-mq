#[macro_use]
extern crate anyhow;

use std::time::Duration;
use std::time::Instant;

use ::futures::prelude::*;

use ::examples::*;

use ::lapin::options::BasicPublishOptions;
use ::lapin::options::ExchangeBindOptions;
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
    let immediate: bool = std::env::var("IMMEDIATE").ok() == Some("1".to_owned());
    let mandatory: bool = std::env::var("MANDATORY").ok() == Some("1".to_owned());
    let message_count: usize = std::env::var("MESSAGE_COUNT").unwrap().parse().unwrap();
    let payload_size: usize = std::env::var("PAYLOAD_SIZE")
        .unwrap_or("1024".to_owned())
        .parse()
        .unwrap();
    let payload: Vec<u8> = vec![0; payload_size];

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
        .queue_declare(
            "q_10_1",
            queue_declare_opts.clone(),
            queue_declare_args.clone(),
        )
        .await?;

    let () = channel_0
        .queue_bind(
            "q_10_1",
            "e_10_1",
            "#",
            queue_bind_opts.clone(),
            queue_bind_args.clone(),
        )
        .await?;

    let start_at = Instant::now();
    for i in 0..message_count {
        let i: usize = i;

        let basic_publish_opts = BasicPublishOptions {
            mandatory,
            immediate,
        };

        let confirm = channel_0
            .basic_publish(
                "e_10_1",
                "please-pretty-please",
                basic_publish_opts,
                payload.to_owned(),
                Default::default(),
            )
            .await?;

        let confirmed = confirm.await?;
        println!("Confirmed #{}: {:?}", i, confirmed);
    }
    let duration = start_at.elapsed();

    let mps = message_count as f64 / duration.as_nanos() as f64 * 1_000_000_000.0;
    println!("duration: {:?}", duration);
    println!("messages: {}", message_count);
    println!("msg/sec:  {}", mps);

    let () = connection.close(200 as u16, "See you!").await?;

    Ok(())
}
