use axum::Router;
use server::routers;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let address = std::env::var("SERVER_URL").expect("SERVER_URL not set");
    let listener = TcpListener::bind(address).await.unwrap();

    let users_router = routers::users_router().await;
    let passwords_router = routers::passwords_router().await;
    let notes_router = routers::notes_router().await;
    let app = Router::new()
        .nest("/users", users_router)
        .nest("/passwords", passwords_router)
        .nest("/notes", notes_router);

    axum::serve(listener, app).await.unwrap();
}
