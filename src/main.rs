use todo_backend::app::build_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = build_router();

    let address = "127.0.0.1:8080";

    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("Listening on {address}");

    axum::serve(listener, app).await?;

    Ok(())
}
