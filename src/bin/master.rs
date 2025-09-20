use ringmote::session::master::Master;

#[tokio::main]
async fn main() {
    let master = Master::new().await;
    master.run().await
}
