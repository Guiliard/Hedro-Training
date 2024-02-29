use crate::infra::AWSMessenger;

use server::{
    io_t_data_services_server::{IoTDataServices, IoTDataServicesServer},
    ListIoTDataRequest, ListIoTDataResponse,
};

struct IoTDataServicesImpl {
    client: AWSMessenger
}

impl IoTDataServicesImpl {
    pub fn new(client: AWSMessenger) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
impl IoTDataServices for IoTDataServicesImpl {
    async fn ListIoTData( &self, _req: tonic::Request<ListIoTDataRequest>, ) -> Result <tonic::Response <ListIoTDataResponse> , tonic::Status> {
        
        //realizar a consulta no timestrem
        // converter o dado obtido no timestream para o tipo criado no protofile IoTData, e adicionar esses valores em um vetor
        // retornar o vetor

        //self.client.query().........
        //conversao do dado
        //
        Ok(tonic::Response::new(ListIoTDataResponse { data: vec![] }))
    }
}