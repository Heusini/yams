use futures::future::join_all;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

pub mod config;

fn test() {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test();
    let mut futs = vec![];
    for _ in 1..10 {
        let output = Command::new("sleep").arg("60").output();
        futs.push(timeout(Duration::from_millis(60000), output));
    }

    let results = join_all(futs).await;
    println!("{}", results.len());
    let mut x = 0;

    for i in &results {
        match i {
            Ok(_) => x += 1,
            Err(_) => (),
        }
    }
    println!("{:?}", results);
    println!("{}", x);
    Ok(())
}
