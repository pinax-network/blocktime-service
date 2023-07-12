extern crate core;

mod pb;

use prost::Message;
use chrono::prelude::*;

use crate::pb::pinax::service::v1::*;
use substreams::proto;
use substreams_sink_kv::prelude::*;

#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn pinax_service_v1_blocktime_blockidbytime(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = BlockIdRequest::decode(&v[..]).expect("Failed to decode");
    let store = Store::new();
    let key: String;

    if let Some(_dt) = req.timestamp.parse::<DateTime<Utc>>().ok() {
        key = format!("block.timestamp:{}", req.timestamp);
    } else {
        match store.prefix(format!("block.timestamp:{}", req.timestamp), Some(1)).pairs.first() {
            Some(kv_pair) => key = kv_pair.key.clone(),
            None => panic!("Date not found in DB: {:?}", req.timestamp) // TODO: Don't crash but inform user via gRPC error response
        }
    }

    let out: BlockId = proto::decode(&store.get(key).unwrap().value).unwrap();

    return Ok(out.encode_to_vec());
}

#[wasmedge_bindgen]
pub fn pinax_service_v1_blocktime_blocktimebyid(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = BlockTimestampRequest::decode(&v[..]).expect("Failed to decode");
    let store = Store::new();

    // TODO: Query by block number or id (requires store change upstream)

    let key = format!("block.number:{}", req.number);
    let out = BlockTimestamp { timestamp: proto::decode(&store.get(key).unwrap().value).unwrap() };

    return Ok(out.encode_to_vec());
}

#[wasmedge_bindgen]
pub fn pinax_service_v1_blocktime_blockrangebydate(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = BlockRangeRequest::decode(&v[..]).expect("Failed to decode");
    let store = Store::new();

    let out = BlockRange { range: vec![] };

    return Ok(out.encode_to_vec());
}