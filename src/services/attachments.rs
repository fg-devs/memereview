use crate::prelude::Res;
use attachments_service::attachments_client::AttachmentsClient;
use attachments_service::NewAttachmentRequest;

use futures::lock::Mutex;
use once_cell::sync::OnceCell;
use std::env;
use tonic::Request;

pub mod attachments_service {
    tonic::include_proto!("attachments");
}

pub static ATTACHMENT_SERVICE: OnceCell<Mutex<Attachments>> = OnceCell::new();

#[derive(Debug)]
pub struct Attachments {
    client: AttachmentsClient<tonic::transport::Channel>,
}

impl Attachments {
    pub async fn new() -> Self {
        let client =
            AttachmentsClient::connect(format!("http://{}", env::var("ATTACHMENTS_URL").unwrap()))
                .await
                .expect("Unable to connect");

        Self { client }
    }

    pub fn set(self) {
        ATTACHMENT_SERVICE.set(Mutex::new(self)).expect("Unable to set ATTACHMENT_SERVICE");
    }

    pub fn global() -> &'static Mutex<Self> {
        ATTACHMENT_SERVICE.get().expect("ATTACHMENT_SERVICE not initialized")
    }

    pub async fn upload(&mut self, url: String) -> Res<String> {
        let request = Request::new(NewAttachmentRequest { url });
        let res = self.client.create_attachment(request).await?.into_inner();

        Ok(res.url)
    }
}
