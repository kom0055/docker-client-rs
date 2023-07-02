use std::collections::HashMap;
use docker_client_rs::engine::client::EngineClient;
use serde::{Deserialize, Serialize};
use docker_client_rs::common::errors::DockerError;
use docker_client_rs::common::types as common_types;
use docker_client_rs::engine::containers::apis::ContainerApi;
use docker_client_rs::engine::containers::types::{Container, ListContainerRequest};
use url::Url;

extern crate serde_qs as qs;
extern crate serde_urlencoded as urlencoded;


#[tokio::test(flavor = "multi_thread")]
async fn test_list_containers() {
    let client = EngineClient::new("/var/run/docker.sock");
    //let a = client.list_containers();
    //print!("result is {:#?}", a);
}


#[tokio::test(flavor = "multi_thread")]
async fn test_encode() {
    let filters = HashMap::from([("1".to_string(), "2".to_string())]);
    let req = ListContainerRequest {
        all: common_types::OptionBool::from(false),
        limit: None,
        size: None,
        filters: common_types::OptionHashMapString2String::from(filters),
    };

    let encoded = serde_urlencoded::to_string(&req);
    println!("{}", encoded.unwrap())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_enum() {
    #[derive(Deserialize, Serialize, Debug)]
    #[serde(rename_all = "lowercase")]
    enum Label {
        AAAA,
        B,
        C,
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct Domain {
        a: i64,
        b: Label,
    }
    let a = Domain {
        a: 2,
        b: Label::B,
    };
    println!("{:#?}", serde_json::to_string(&a));

    let s = "{\"a\":2,\"b\":\"c\"}";
    let g: serde_json::Result<Domain> = serde_json::from_str(s);
    println!("{:#?}", g)
}