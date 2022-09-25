use ::examples::*;

use ::lapin::options::QueueDeclareOptions;
use ::lapin::types::FieldTable;
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

    let queue_name_1 = std::env::var("QUEUE_NAME_1").unwrap_or_else(|_| "".to_owned());
    let queue_name_2 = std::env::var("QUEUE_NAME_2").unwrap_or_else(|_| "".to_owned());

    let connection = Connection::connect(&amqp_uri, Default::default()).await?;
    let channel_0 = connection.create_channel().await?;
    let channel_1 = connection.create_channel().await?;

    let queue_declare_opts = QueueDeclareOptions::default();
    let queue_declare_args = FieldTable::default();

    let queue_1 = channel_0
        .queue_declare(&queue_name_1, queue_declare_opts.clone(), queue_declare_args.clone())
        .await?;
    log::info!("q1: {:?}", queue_1.name());

    let queue_2 = channel_1
        .queue_declare(&queue_name_2, queue_declare_opts.clone(), queue_declare_args.clone())
        .await?;
    log::info!("q2: {:?}", queue_2.name());

    let _ = channel_1.queue_delete(&queue_1.name().as_str(), Default::default()).await?;
    let _ = channel_0.queue_delete(&queue_2.name().as_str(), Default::default()).await?;

    let () = connection.close(200 as u16, "See you!").await?;

    Ok(())
}
