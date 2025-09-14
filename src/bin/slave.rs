use ringmote::session::slave::Slave;

#[tokio::main]
async fn main() {
    let slave = Slave::new().await;
    slave.run().await
}
