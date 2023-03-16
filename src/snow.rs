use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Result, Error};

// 起始时间
const START_TIME: u128 = 1659283200000;
// 节点ID所占的位数
const WORKER_ID_BITS: u128 = 5;
// 数据中心ID所占的位数
const DATA_CENTER_ID_BITS: u128 = 5;
// 最大的节点ID
const MAX_WORKER_ID: u128 = (-1 ^ (-1 << WORKER_ID_BITS)) as u128;
// 最大的数据中心ID
const MAX_DATA_CENTER_ID: u128 = (-1 ^ (-1 << DATA_CENTER_ID_BITS)) as u128;
// 序列号所占的位数
const SEQUENCE_BITS: u128 = 12;
// 节点偏移
const WORKER_ID_SHIFT: u128 = SEQUENCE_BITS;
// 数据中心偏移
const DATA_CENTER_ID_SHIFT: u128 = SEQUENCE_BITS + WORKER_ID_BITS;
// 时间戳偏移
const TIMESTAMP_LEFT_SHIFT: u128 = SEQUENCE_BITS + WORKER_ID_BITS + DATA_CENTER_ID_BITS;
// 序列掩码
const SEQUENCE_MASK: u128 = (-1 ^ (-1 << SEQUENCE_BITS)) as u128;

#[derive(Clone)]
pub struct SnowflakeIdWorker(Arc<Mutex<SnowflakeIdWorkerInner>>);

struct SnowflakeIdWorkerInner {
    worker_id: u128,
    data_center_id: u128,
    sequence: u128,
    last_timestamp: u128,
}

impl SnowflakeIdWorker {
    pub fn new(worker_id: u128, data_center_id: u128) -> Result<SnowflakeIdWorker> {
        Ok(
            Self(Arc::new(Mutex::new(SnowflakeIdWorkerInner::new(worker_id, data_center_id)?)))
        )
    }

    pub fn next_id(&self) -> Result<u128> {
        let mut inner = self.0.lock().map_err(|e| Error::msg(e.to_string()))?;
        inner.next_id()
    }
}

impl SnowflakeIdWorkerInner {
    fn new(worker_id: u128, data_center_id: u128) -> Result<SnowflakeIdWorkerInner> {
        if worker_id > MAX_WORKER_ID {
            return Err(Error::msg(format!("workerId:{} must be less than {}", worker_id, MAX_WORKER_ID)));
        }
        if data_center_id > MAX_DATA_CENTER_ID {
            return Err(Error::msg(format!("datacenterId:{} must be less than {}", data_center_id, MAX_DATA_CENTER_ID)));
        }
        Ok(SnowflakeIdWorkerInner {
            worker_id,
            data_center_id,
            sequence: 0,
            last_timestamp: 0,
        })
    }

    fn next_id(&mut self) -> Result<u128> {
        let mut timestamp = Self::get_time()?;
        if timestamp < self.last_timestamp {
            return Err(Error::msg(format!("Clock moved backwards.  Refusing to generate id for {} milliseconds", self.last_timestamp - timestamp)));
        }
        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & SEQUENCE_MASK;
            if self.sequence == 0 {
                timestamp = Self::til_next_mills(self.last_timestamp)?;
            }
        } else {
            self.sequence = 0;
        }
        self.last_timestamp = timestamp;
        Ok(((timestamp - START_TIME) << TIMESTAMP_LEFT_SHIFT)
            | (self.data_center_id << DATA_CENTER_ID_SHIFT)
            | (self.worker_id << WORKER_ID_SHIFT)
            | self.sequence)
    }

    fn til_next_mills(last_timestamp: u128) -> Result<u128> {
        let mut timestamp = Self::get_time()?;
        while timestamp <= last_timestamp {
            timestamp = Self::get_time()?;
        }
        Ok(timestamp)
    }

    fn get_time() -> Result<u128> {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(s) => {
                Ok(s.as_millis())
            }
            Err(_) => {
                Err(Error::msg("get_time error!"))
            }
        }
    }
}