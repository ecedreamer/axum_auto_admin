use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use axum::Router;
use axum::routing::get;
use axum_auto_admin_derive::DataModel;
use axum_auto_admin::DataModel;


#[derive(DataModel, Debug)]
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


async fn get_country_detail(Path(country_name): Path<String>) -> impl IntoResponse {
    Html::from(
        format!("<h3>{}</h3> is one of the most beautiful country", country_name)
    )
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/country/{country_name}", get(get_country_detail))
        .nest("/admin/", Post::get_router())
        .nest("/admin/", User::get_router());


    let user = User {
        email: "sangit.niroula@gmail.com".to_string(),
        name: "Sangit Niroula".to_string(),
    };

    println!("User: {:?}", user);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::info!("Listening on http://0.0.0.0:8000");
    axum::serve(listener, app).await.unwrap();
}
