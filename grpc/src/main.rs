pub mod server { tonic::include_proto!("server"); }

use std::error::Error;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    
    let addr = "0.0.0.0:50051".parse()?; //host e porta do server

    //fazemos a conexao com o timestrem, usando o sdk da aws na crate timestreamquery;
    // let client = .........

    let service = IoTDataServicesImpl::new(
        //client
);

    info!("Staging grpc server....");

    Server::builder()
        .add_service(IoTDataServicesServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}