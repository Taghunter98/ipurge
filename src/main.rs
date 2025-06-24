pub mod purge;

#[tokio::main]
async fn main() {
    purge::run().await;
}
