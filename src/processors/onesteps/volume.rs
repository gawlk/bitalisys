use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::NaiveDate;
use std::marker::PhantomData;

use crate::{
    output::*,
    processors::{DBCaches, DailyBlocksProcessor, Processor},
};

pub struct VolumeCounter;

pub type VolumeCounterProcessor = Processor<u64, VolumeCounter>;

impl VolumeCounterProcessor {
    pub fn new(path: &str) -> Self {
        Self {
            name: "Counter/Volume".to_string(),
            output: Output::new(path, "counters/volume.json"),
            pd: PhantomData,
        }
    }
}

impl DailyBlocksProcessor<u64> for VolumeCounterProcessor {
    fn process_daily_blocks(
        &self,
        blocks: &[FBlock],
        db: &BitcoinDB,
        caches: &DBCaches,
        _: &NaiveDate,
    ) -> u64 {
        blocks
            .iter()
            .flat_map(|block| {
                block.txdata.iter().flat_map(|tx| {
                    tx.input.iter().map(|txin| {
                        let txid = txin.previous_output.txid;

                        let vout = txin.previous_output.vout;

                        self.outpoint_to_value(txid, vout, db, caches)
                    })
                })
            })
            .sum()
    }
}
