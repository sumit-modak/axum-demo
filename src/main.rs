use tokio;

#[tokio::main]
async fn main() {
    let app = axum_demo::routes::all_routes();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("[-] listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
