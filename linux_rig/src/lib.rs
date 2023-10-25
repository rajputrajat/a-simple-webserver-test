#[tarpc::service]
pub trait RigInfo {
    async fn ip() -> String;
}
