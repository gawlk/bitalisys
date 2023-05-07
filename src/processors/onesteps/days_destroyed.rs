use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::NaiveDate;
use std::marker::PhantomData;

use crate::{
    output::*,
    processors::{DBCaches, DailyBlocksProcessor, Processor},
};

pub struct DaysDestroyedCounter;

pub type DaysDestroyedCounterProcessor = Processor<f64, DaysDestroyedCounter>;

impl DaysDestroyedCounterProcessor {
    pub fn new(path: &str) -> Self {
        Self {
            name: "Counter/DaysDestroyed".to_string(),
            output: Output::new(path, "counters/days_destroyed.json"),
            pd: PhantomData,
        }
    }
}

impl DailyBlocksProcessor<f64> for DaysDestroyedCounterProcessor {
    fn process_daily_blocks(
        &self,
        blocks: &[FBlock],
        db: &BitcoinDB,
        caches: &DBCaches,
        date: &NaiveDate,
    ) -> f64 {
        blocks
            .iter()
            .flat_map(|block| {
                block.txdata.iter().flat_map(|tx| {
                    tx.input.iter().map(|txin| {
                        let txid = txin.previous_output.txid;

                        let vout = txin.previous_output.vout;

                        let value = self.outpoint_to_value(txid, vout, db, caches);

                        let bitcoins = (value as f64) / 100_000_000.0;

                        let prev_date = self.txid_to_naive_date(txid, db, caches);

                        let num_days = date.signed_duration_since(prev_date).num_days() as f64;

                        bitcoins * num_days
                    })
                })
            })
            .sum()
    }
}
