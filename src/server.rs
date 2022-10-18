mod lib;

use date_rpc::date_service_server::DateServiceServer;
use tonic::{transport::Server};
use lib::{MyDateService, date_rpc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:19813".parse()?;
    let date_server = MyDateService::new("token.txt".to_string());

    Server::builder()
        .add_service(DateServiceServer::new(date_server))
        .serve(addr)
        .await?;

    Ok(())
}