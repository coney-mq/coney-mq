use std::env;

pub fn amqp_uri() -> String {
    env::var("AMQP_URI").unwrap_or("amqp://guest:guest@127.0.0.1:5672/".to_owned())
}
pub fn app_id() -> String {
    env::var("APP_ID").unwrap_or("example-app".to_owned())
}
