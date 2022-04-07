use serde::{Deserialize};
use std::collections::HashMap;


#[derive(Deserialize, Debug)]
pub struct Container {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Names")]
    pub names: Vec<String>,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "ImageID")]
    pub image_id: String,
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "Created")]
    pub created: i64,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Ports")]
    pub ports: Vec<Port>,
    #[serde(rename = "Labels")]
    pub labels: HashMap<String, String>,
    #[serde(rename = "SizeRw", skip_serializing_if = "Option::is_none")]
    pub size_rw: Option<i64>,
    #[serde(rename = "SizeRootFs", skip_serializing_if = "Option::is_none")]
    pub size_root_fs: Option<i64>,
    #[serde(rename = "HostConfig")]
    pub host_config: self::HostConfig,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: self::NetworkSettings,
    #[serde(rename = "Mounts")]
    pub mounts: Vec<self::Mounts>,
}

#[derive(Deserialize, Debug)]
pub struct Port {
    #[serde(rename = "PrivatePort")]
    pub private_port: i64,
    #[serde(rename = "PublicPort", skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i64>,
    #[serde(rename = "Type")]
    pub protocol_type: String,
}

#[derive(Deserialize, Debug)]
pub struct HostConfig {
    #[serde(rename = "NetworkMode")]
    pub network_mode: String,
}


#[derive(Deserialize, Debug)]
pub struct NetworkSettings {
    #[serde(rename = "Networks")]
    pub networks: self::Networks,
}

#[derive(Deserialize, Debug)]
pub struct Networks {
    #[serde(rename = "Bridge", skip_serializing_if = "Option::is_none")]
    pub bridge: Option<self::Bridge>,
}

#[derive(Deserialize, Debug)]
pub struct Bridge {
    #[serde(rename = "NetworkID")]
    pub network_id: String,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    #[serde(rename = "Gateway")]
    pub gateway: String,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: i64,
    #[serde(rename = "IPv6Gateway")]
    pub ip_v6_gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ip_v6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ip_v6_prefix_len: i64,
    #[serde(rename = "MacAddress")]
    pub mac_address: String,
}

#[derive(Deserialize, Debug)]
pub struct Mounts {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "RW")]
    pub rw: bool,
    #[serde(rename = "Propagation")]
    pub propagation: String,
}