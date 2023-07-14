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

    match store.prefix(prefix.clone(), Some(1)).pairs.first() {
        Some(kv_pair) => Ok(kv_pair.key.clone()),
        None => Err(format!("Prefix not found in DB: {:?}", prefix))
    }
}

fn safe_get_from_store(key: String) -> Result<Vec<u8>, String> {
    let store = Store::new();

    match store.get(key.clone()) {
        Some(out) => Ok(out.value),
        None => Err(format!("Key not found in DB: {:?}", key))
    }
}

#[wasmedge_bindgen]
pub fn pinax_service_v1_blocktime_blockidbytime(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = BlockIdRequest::decode(&v[..]).expect("[BlockIdRequest] Failed to decode");

    let key = match req.timestamp.parse::<DateTime<Utc>>() {
        Ok(_) => format!("block.timestamp:{}", req.timestamp),
        Err(_) => get_key_by_prefix(format!("block.timestamp:{}", req.timestamp))?
    };

    safe_get_from_store(key)
}

#[wasmedge_bindgen]
pub fn pinax_service_v1_blocktime_blocktimebyid(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = BlockTimestampRequest::decode(&v[..]).expect("[BlockTimestampRequest] Failed to decode");

    // TODO: Query by block number or id (requires store change upstream)

    let key = format!("block.number:{}", req.number);

    safe_get_from_store(key)
}

#[wasmedge_bindgen]
pub fn pinax_service_v1_blocktime_blockrangebydate(v: Vec<u8>) -> Result<Vec<u8>, String> {
    let req = BlockRangeRequest::decode(&v[..]).expect("[BlockRangeRequest] Failed to decode");

    let first_key = get_key_by_prefix(format!("block.timestamp:{}", req.first_date))?;
    let second_key = match req.second_date {
        Some(second_date) => get_key_by_prefix(format!("block.timestamp:{}", second_date))?,
        None => {
            let datetime_utc = match NaiveDate::parse_from_str(&req.first_date, "%Y-%m-%d") {
                // Try to parse naive date
                Ok(naive_date) => Ok(DateTime::<Utc>::from_utc(naive_date.and_hms_opt(0, 0, 0).unwrap(), Utc)),
                // Else assume we got a full timestamp
                Err(_) => req.first_date.parse::<DateTime<Utc>>().map_err(
                    |e| format!("{:?} -- Wrong `first_date` format, expecting date or timestamp: {:?}", e, req.first_date)
                )
            }?;
            // Get next day after `first_date`
            let next_day_dt = datetime_utc + Duration::days(1);

            get_key_by_prefix(format!("block.timestamp:{}", next_day_dt.format("%Y-%m-%d").to_string()))?
        }
    };

    let out = BlockRange {
        range: vec![
            proto::decode::<BlockId>(&safe_get_from_store(first_key)?).unwrap(),
            proto::decode::<BlockId>(&safe_get_from_store(second_key)?).unwrap()
        ]
    };

    return Ok(out.encode_to_vec());
}