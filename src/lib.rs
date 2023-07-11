extern crate core;

mod pb;

use prost::Message;

use crate::pb::pinax::service::v1::{BlockIdRequest, BlockTimestampRequest, BlockId, BlockTimestamp};
use substreams::proto;
use substreams_sink_kv::prelude::*;

#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

#[wasmedge_bindgen]
pub fn pinax_service_v1_blocktime_blockidbytime(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = BlockIdRequest::decode(&v[..]).expect("Failed to decode");
    let store = Store::new();

    // TODO: Find closest timestamp

    let key = format!("block.timestamp:{}", req.timestamp);
    let out: BlockId = proto::decode(&store.get(key).unwrap().value).unwrap();//BlockId { id: 1, number:  };

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
