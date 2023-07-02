use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::fmt::{Debug, Display, Formatter};

use chrono::serde::{ts_seconds, ts_seconds_option};

pub type OptionBool = Option<bool>;


pub type OptionString = Option<String>;
pub type OptionVecString = Option<Vec<String>>;
pub type OptionDatetime = Option<chrono::DateTime<chrono::Utc>>;
pub type OptionI64 = Option<i64>;
pub type OptionU16 = Option<u16>;
pub type OptionHashMapString2String = Option<HashMap<String, String>>;
pub type OptionIpAddr = Option<IpAddr>;