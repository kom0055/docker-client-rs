use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use chrono::serde::{ts_seconds, ts_seconds_option};
use structmap::{ToMap, value::Value, StringMap, GenericMap};


use std::collections::BTreeMap;
use crate::common::types as common_types;

type OptionNetProtocol = Option<self::NetProtocol>;
type OptionVecPort = Option<Vec<self::Port>>;
type OptionHostConfig = Option<self::HostConfig>;
type OptionNetworkSettings = Option<self::NetworkSettings>;
type OptionMapString2Network = Option<HashMap<String, self::Network>>;
type OptionIPAMConfig = Option<self::IPAMConfig>;
type OptionVecMount = Option<Vec<self::Mount>>;


#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ListContainerRequest {
    pub all: common_types::OptionBool,
    pub limit: common_types::OptionI64,
    pub size: common_types::OptionBool,
    pub filters: common_types::OptionHashMapString2String,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct ContainersFilter {}


#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    pub id: common_types::OptionString,
    pub names: common_types::OptionVecString,
    pub image: common_types::OptionString,
    #[serde(rename = "ImageID")]
    pub image_id: common_types::OptionString,
    pub command: common_types::OptionString,
    #[serde(with = "ts_seconds_option")]
    pub created: common_types::OptionDatetime,
    pub ports: self::OptionVecPort,
    pub size_rw: common_types::OptionI64,
    pub size_root_fs: common_types::OptionI64,
    pub labels: common_types::OptionHashMapString2String,
    pub state: common_types::OptionString,
    pub status: common_types::OptionString,
    pub host_config: self::OptionHostConfig,
    pub network_settings: self::OptionNetworkSettings,
    //#[serde(flatten)]
    pub mounts: self::OptionVecMount,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NetProtocol {
    TCP,
    UDP,
    SCTP,
}


#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    #[serde(rename = "IP")]
    pub ip: common_types::OptionIpAddr,
    pub private_port: common_types::OptionU16,
    pub public_port: common_types::OptionU16,
    #[serde(rename = "Type")]
    pub protocol_type: self::NetProtocol,
}


#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    pub network_mode: common_types::OptionString,
}


#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
    pub networks: self::OptionMapString2Network,
}


#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Network {
    #[serde(rename = "IPAMConfig")]
    pub ip_am_config: self::OptionIPAMConfig,
    pub links: common_types::OptionVecString,
    pub aliases: common_types::OptionVecString,
    #[serde(rename = "NetworkID")]
    pub network_id: common_types::OptionString,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: common_types::OptionString,
    pub gateway: common_types::OptionString,
    #[serde(rename = "IPAddress")]
    pub ip_address: common_types::OptionString,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: common_types::OptionU16,
    #[serde(rename = "IPv6Gateway")]
    pub ip_v6_gateway: common_types::OptionString,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ip_v6_address: common_types::OptionString,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ip_v6_prefix_len: common_types::OptionU16,
    pub driver_opts: common_types::OptionHashMapString2String,
    pub mac_address: common_types::OptionString,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IPAMConfig {
    #[serde(rename = "IPv4Address")]
    pub ip_v4_address: common_types::OptionString,
    #[serde(rename = "IPv6Address")]
    pub ip_v6_address: common_types::OptionString,
    #[serde(rename = "LinkLocalIPs")]
    pub link_local_ips: common_types::OptionVecString,
}


#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    pub name: common_types::OptionString,
    pub source: common_types::OptionString,
    pub destination: common_types::OptionString,
    pub driver: common_types::OptionString,
    pub mode: common_types::OptionString,
    #[serde(rename = "RW")]
    pub rw: common_types::OptionBool,
    pub propagation: common_types::OptionString,
}