use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use axum_auto_admin_derive::DataModel;
use axum_auto_admin::DataModel;


#[derive(DataModel)]
struct User {
    name: String,
    email: String,
}


#[derive(DataModel, Debug)]
struct Post {
    title: String,
    image: String,
    content: String,
}

async fn root() -> impl IntoResponse {
    tracing::info!("Root!");
    "This is the home page"
}



#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .nest("/", Post::get_router());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::info!("Listening on http://0.0.0.0:8000");
    axum::serve(listener, app).await.unwrap();

}
