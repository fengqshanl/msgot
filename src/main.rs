mod msgot;
mod client;

use crate::msgot::msgot::msgot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    msgot().await
}
