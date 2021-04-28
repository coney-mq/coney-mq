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
    let args = std::env::args().collect::<Vec<_>>();
    let routing_key = if args.len() > 1 { &args[1] } else { "" };

    let mut mandatory = false;
    let mut immediate = false;

    for s in &args[2..] {
        if s == "i" {
            immediate = true
        }
        if s == "m" {
            mandatory = true
        }
    }

    println!("routing-key: {:?}", routing_key);

    let amqp_uri = config::amqp_uri();

    let connection = Connection::connect(&amqp_uri, Default::default()).await?;
    let channel_0 = connection.create_channel().await?;

    let exchange_kind = ExchangeKind::Topic;

    let exchange_declare_opts = ExchangeDeclareOptions::default();
    let exchange_declare_args = FieldTable::default();

    let queue_declare_opts = QueueDeclareOptions::default();
    let queue_declare_args = FieldTable::default();

    let exchange_bind_opts = ExchangeBindOptions::default();
    let exchange_bind_args = FieldTable::default();

    let queue_bind_opts = QueueBindOptions::default();
    let queue_bind_args = FieldTable::default();

    let () = channel_0
        .exchange_declare(
            "e_09_1",
            exchange_kind.clone(),
            exchange_declare_opts.clone(),
            exchange_declare_args.clone(),
        )
        .await?;
    let () = channel_0
        .exchange_declare(
            "e_09_2_1",
            exchange_kind.clone(),
            exchange_declare_opts.clone(),
            exchange_declare_args.clone(),
        )
        .await?;
    let () = channel_0
        .exchange_declare(
            "e_09_2_2",
            exchange_kind.clone(),
            exchange_declare_opts.clone(),
            exchange_declare_args.clone(),
        )
        .await?;
    let () = channel_0
        .exchange_declare(
            "e_09_3",
            exchange_kind.clone(),
            exchange_declare_opts.clone(),
            exchange_declare_args.clone(),
        )
        .await?;

    let _queue = channel_0
        .queue_declare(
            "q_09_1",
            queue_declare_opts.clone(),
            queue_declare_args.clone(),
        )
        .await?;
    let _queue = channel_0
        .queue_declare(
            "q_09_2_1",
            queue_declare_opts.clone(),
            queue_declare_args.clone(),
        )
        .await?;
    let _queue = channel_0
        .queue_declare(
            "q_09_2_2",
            queue_declare_opts.clone(),
            queue_declare_args.clone(),
        )
        .await?;
    let _queue = channel_0
        .queue_declare(
            "q_09_3",
            queue_declare_opts.clone(),
            queue_declare_args.clone(),
        )
        .await?;

    let () = channel_0
        .exchange_bind(
            "e_09_2_1",
            "e_09_1",
            "#.i2_1.#",
            exchange_bind_opts.clone(),
            exchange_bind_args.clone(),
        )
        .await?;
    let () = channel_0
        .exchange_bind(
            "e_09_2_2",
            "e_09_1",
            "#.i2_2.#",
            exchange_bind_opts.clone(),
            exchange_bind_args.clone(),
        )
        .await?;
    let () = channel_0
        .exchange_bind(
            "e_09_3",
            "e_09_2_1",
            "#.o2_1.#",
            exchange_bind_opts.clone(),
            exchange_bind_args.clone(),
        )
        .await?;
    let () = channel_0
        .exchange_bind(
            "e_09_3",
            "e_09_2_2",
            "#.o2_2.#",
            exchange_bind_opts.clone(),
            exchange_bind_args.clone(),
        )
        .await?;

    let () = channel_0
        .queue_bind(
            "q_09_1",
            "e_09_1",
            "#.d1.#",
            queue_bind_opts.clone(),
            queue_bind_args.clone(),
        )
        .await?;
    let () = channel_0
        .queue_bind(
            "q_09_2_1",
            "e_09_2_1",
            "#.d2_1.#",
            queue_bind_opts.clone(),
            queue_bind_args.clone(),
        )
        .await?;
    let () = channel_0
        .queue_bind(
            "q_09_2_2",
            "e_09_2_2",
            "#.d2_2.#",
            queue_bind_opts.clone(),
            queue_bind_args.clone(),
        )
        .await?;
    let () = channel_0
        .queue_bind(
            "q_09_3",
            "e_09_3",
            "#.d3.#",
            queue_bind_opts.clone(),
            queue_bind_args.clone(),
        )
        .await?;

    let basic_publish_opts = BasicPublishOptions {
        mandatory,
        immediate,
    };

    let confirm = channel_0
        .basic_publish(
            "e_09_1",
            &routing_key,
            basic_publish_opts,
            Vec::new(),
            Default::default(),
        )
        .await?;

    let confirmed = confirm.await?;

    println!("Confirmed: {:#?}", confirmed);

    let () = channel_0
        .exchange_declare(
            "e_09_1",
            exchange_kind.clone(),
            exchange_declare_opts.clone(),
            exchange_declare_args.clone(),
        )
        .await?;

    let () = connection.close(200 as u16, "See you!").await?;

    Ok(())
}
