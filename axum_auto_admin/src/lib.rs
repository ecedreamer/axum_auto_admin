use axum::extract::Path;

pub trait DataModel {
    fn get_struct_name() -> String;
    fn get_router() -> axum::Router;
    fn list_handler() -> impl std::future::Future<Output = impl axum::response::IntoResponse> + Send;
    fn create_handler() -> impl std::future::Future<Output = impl axum::response::IntoResponse> + Send;
    fn create_handler_post() -> impl std::future::Future<Output = impl axum::response::IntoResponse> + Send;
    fn update_handler(_: Path<usize>) -> impl std::future::Future<Output = impl axum::response::IntoResponse> + Send;
    fn update_handler_post(_: Path<usize>) -> impl std::future::Future<Output = impl axum::response::IntoResponse> + Send;
    fn delete_handler(_: Path<usize>) -> impl std::future::Future<Output = impl axum::response::IntoResponse> + Send;
    fn delete_handler_post(_: Path<usize>) -> impl std::future::Future<Output = impl axum::response::IntoResponse> + Send;
}