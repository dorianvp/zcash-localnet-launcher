use std::sync::{Arc, Mutex};

use local_net::{LocalNet, indexer::zainod::Zainod, process::Process, validator::zebrad::Zebrad};
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() {
    let network = LocalNet::<Zebrad, Zainod>::launch_default().await.unwrap();

    println!("Indexer running at: 127.0.0.1:{}", network.indexer().port());

    let should_continue = Arc::new(Mutex::new(true));
    let should_continue_clone = Arc::clone(&should_continue);

    tokio::spawn(async move {
        loop {
            if *should_continue_clone.lock().unwrap() {
                network.print_all();
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            } else {
                break;
            }
        }
    });

    ctrl_c().await.expect("Failed to listen for Ctrl+C signal");
    *should_continue.lock().unwrap() = false;
}
