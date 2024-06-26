use axum::Router;
use server_lib::routers;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let users_router = routers::users_router();
    let passwords_router = routers::passwords_router();
    let notes_router = routers::notes_router();
    let app = Router::new()
        .nest("/users", users_router.await)
        .nest("/passwords", passwords_router.await)
        .nest("/notes", notes_router.await);

    let address = std::env::var("SERVER_URL").expect("SERVER_URL not set");
    let listener = TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
