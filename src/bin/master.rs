use std::time::Duration;

use ringmote::session::backend::tcp::TcpMaster;

#[tokio::main]
async fn main() {
    std::thread::sleep(Duration::from_millis(2000));
    let master = TcpMaster::new().await;
    master.run().await
}
