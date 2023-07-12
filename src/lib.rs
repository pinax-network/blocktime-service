extern crate core;

mod pb;

use prost::Message;
use chrono::*;

use crate::pb::pinax::service::v1::*;
use substreams::proto;
use substreams_sink_kv::prelude::*;

#[allow(unused_imports)]
use wasmedge_bindgen::*;
use wasmedge_bindgen_macro::*;

fn get_key_by_prefix(prefix: String) -> Result<String, String> {
    let store = Store::new();
    let key: String;

    match store.prefix(prefix.clone(), Some(1)).pairs.first() {
        Some(kv_pair) => key = kv_pair.key.clone(),
        None => panic!("Date not found in DB: {:?}", prefix) // TODO: Don't crash but inform user via gRPC error response
    }

    Ok(key)
}

#[wasmedge_bindgen]
pub fn pinax_service_v1_blocktime_blockidbytime(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = BlockIdRequest::decode(&v[..]).expect("Failed to decode");
    let store = Store::new();
    let key: String;

    if let Some(_dt) = req.timestamp.parse::<DateTime<Utc>>().ok() {
        key = format!("block.timestamp:{}", req.timestamp);
    } else {
        key = get_key_by_prefix(format!("block.timestamp:{}", req.timestamp)).unwrap()
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
    let first_key: String;
    let second_key: String;

    first_key = get_key_by_prefix(format!("block.timestamp:{}", req.first_date)).unwrap();

    if let Some(second_date) = req.second_date {
        second_key = get_key_by_prefix(format!("block.timestamp:{}", second_date)).unwrap();
    } else {
        let naive_date = NaiveDate::parse_from_str(&req.first_date, "%Y-%m-%d").unwrap();
        let naive_datetime: NaiveDateTime = naive_date.and_hms_opt(0, 0, 0).unwrap();
        let datetime_utc = DateTime::<Utc>::from_utc(naive_datetime, Utc);
        let next_day_dt = datetime_utc + Duration::days(1);
        
        second_key = get_key_by_prefix(format!("block.timestamp:{}", next_day_dt.format("%Y-%m-%d").to_string())).unwrap();
    }

    let out = BlockRange {
        range: vec![
            proto::decode(&store.get(first_key).unwrap().value).unwrap(),
            proto::decode(&store.get(second_key).unwrap().value).unwrap()
        ]
    };

    return Ok(out.encode_to_vec());
}