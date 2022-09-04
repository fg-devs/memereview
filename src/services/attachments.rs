use crate::prelude::Res;
use attachments_service::attachments_client::AttachmentsClient;
use attachments_service::NewAttachmentRequest;
use std::env;
use tonic::Request;

pub mod attachments_service {
    tonic::include_proto!("attachments");
}

pub struct Attachments {
    client: AttachmentsClient<tonic::transport::Channel>,
}

impl Attachments {
    pub async fn new() -> Self {
        let client = AttachmentsClient::connect(
            env::var("ATTACHMENTS_URL").expect("Env ATTACHMENTS_URL missing"),
        )
        .await
        .expect("Unable to connect");

        Self { client }
    }

    pub async fn upload(&mut self, url: String) -> Res<String> {
        let request = Request::new(NewAttachmentRequest { url });
        let res = self.client.create_attachment(request).await?.into_inner();

        Ok(res.url)
    }
}
