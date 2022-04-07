extern crate tokio;


use std::fmt::{Debug};
use crate::engine::containers::apis::{ContainerApi};
use serde::{Deserialize};
use hyper::{Client, Request};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use crate::engine::containers::types::{Container};
use futures::stream::Concat;
use futures::{TryFutureExt};
use futures::executor::block_on;
use hyper::body::Buf;
pub use crate::common::errors;


#[derive(Debug)]
pub struct EngineClient {
    socket: String,
    client: Client<UnixConnector, hyper::Body>,
}


impl EngineClient {
    pub fn new<T>(path: T) -> EngineClient where T: Into<String> {
        EngineClient {
            socket: path.into(),
            client: Client::unix(),
        }
    }

    async fn async_request<T: for<'de> Deserialize<'de>>(&self, request: hyper::Request<hyper::Body>) -> Result<T, errors::DockerError> {
        let client = self.client.clone();
        let res = client.request(request).map_err(errors::ErrorMessage::map_err_hyper).await?;
        let status = res.status();
        let body = hyper::body::aggregate(res).map_err(errors::ErrorMessage::map_err_hyper).await?;


        if !status.is_success() {
            let v = serde_json::from_reader(body.reader()).map_err(errors::ErrorMessage::map_err_serde_json)?;
            return match status.as_u16() {
                400 => Err(errors::DockerError::BadParameters(v)),
                404 => Err(errors::DockerError::NotFound(v)),
                409 => Err(errors::DockerError::NotRunning(v)),
                500 => Err(errors::DockerError::ServerError(v)),
                _ => Err(errors::DockerError::UnknownStatus(v)),
            };
        }

        let v = serde_json::from_reader(body.reader()).map_err(errors::ErrorMessage::map_err_serde_json)?;
        Ok(v)
    }
    fn request<T: for<'de> Deserialize<'de>>(&self, request: hyper::Request<hyper::Body>) -> Result<T, errors::DockerError> {
        let future = self.async_request(request);
        block_on(future)
    }
}

pub struct DockerFuture {
    pub status: hyper::StatusCode,
    pub body: Concat<hyper::Body>,
}


impl ContainerApi for EngineClient {
    fn list_containers(&self) -> Result<Vec<Container>, errors::DockerError> {
        let path = "/containers/json";

        let url: hyper::Uri = Uri::new(self.socket.as_str(), path).into();
        let request = Request::get(url)
            .header("Content-Type", "application/json").body(hyper::Body::empty()).unwrap();
        let res = self.request(request);
        return res;
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_list_containers() {
    let client = EngineClient::new("/var/run/docker.sock");
    let a = client.list_containers();
    println!("rect1 is {:#?}", a);
}