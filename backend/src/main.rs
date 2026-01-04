use dissipate::handlers;

#[tokio::main]
async fn main() {
    handlers::run_server().await;
}
