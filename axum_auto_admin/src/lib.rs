


pub trait DataModel {
    fn get_struct_name() -> String;
    fn get_router() -> axum::Router;
    fn list_handler() -> impl std::future::Future<Output = impl axum::response::IntoResponse> + Send;
    fn create_handler() -> impl std::future::Future<Output = impl axum::response::IntoResponse> + Send;
    fn update_handler() -> impl std::future::Future<Output = impl axum::response::IntoResponse> + Send;
    fn delete_handler() -> impl std::future::Future<Output = impl axum::response::IntoResponse> + Send;
}