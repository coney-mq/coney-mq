use std::time::Instant;

use ::futures::prelude::*;

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
    let message_count: usize = std::env::var("MESSAGE_COUNT").unwrap().parse().unwrap();
    let prefetch_count: usize = std::env::var("PREFETCH_COUNT")
        .unwrap_or("0".to_owned())
        .parse()
        .unwrap();

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

    if prefetch_count > 0 {
        let () = channel_0
            .basic_qos(prefetch_count as u16, Default::default())
            .await?;
    }

    let mut consumer = channel_0
        .basic_consume(
            queue.name().as_str(),
            "consume-q_10_1",
            Default::default(),
            Default::default(),
        )
        .await?;

    println!("Consumer: {:#?}", consumer);

    let mut start_at: Option<Instant> = None;
    let mut messages_received: usize = 0;

    for i in 0..message_count {
        if let Some(delivery) = consumer.next().await {
            if start_at.is_none() {
                start_at = Some(Instant::now());
            }

            let (consumer_channel, delivery) = delivery?;
            println!("Delivery #{}: {} bytes", i, delivery.data.len());

            if prefetch_count > 0 {
                let () = consumer_channel
                    .basic_ack(delivery.delivery_tag, Default::default())
                    .await?;
            }

            messages_received = messages_received + 1;
        } else {
            break;
        }
    }

    if let Some(start_at) = start_at {
        if messages_received != 0 {
            let duration = start_at.elapsed();

            let mps = messages_received as f64 / duration.as_nanos() as f64 * 1_000_000_000.0;
            println!("duration: {:?}", duration);
            println!("messages: {}", messages_received);
            println!("msg/sec:  {}", mps);
        }
    }

    let () = connection.close(200 as u16, "See you!").await?;

    Ok(())
}
