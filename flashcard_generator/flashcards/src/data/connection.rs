#[path = "./env_conn.rs"] mod environment;
extern crate redis;

pub fn connect() -> redis::Connection {
    let redis_conn_string = format!("redis://default:{}@{}:{}", environment::REDIS_PASS, environment::REDIS_HOST, environment::REDIS_PORT);

    let client = redis::Client::open(redis_conn_string)
        .expect("Invalid connection information");
    let connection = client.get_connection()
        .expect("Failed to establish connection");

    return connection;
}