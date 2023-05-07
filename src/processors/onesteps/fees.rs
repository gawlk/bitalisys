use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::NaiveDate;
use std::marker::PhantomData;

use crate::{
    output::*,
    processors::{DBCaches, DailyBlocksProcessor, Processor},
};

pub struct FeesCounter;

pub type FeesCounterProcessor = Processor<u64, FeesCounter>;

impl FeesCounterProcessor {
    pub fn new(path: &str) -> Self {
        Self {
            name: "Counter/Fees".to_string(),
            output: Output::new(path, "counters/fees.json"),
            pd: PhantomData,
        }
    }
}

impl DailyBlocksProcessor<u64> for FeesCounterProcessor {
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
                block
                    .txdata
                    .iter()
                    .filter(|tx| !tx.input.is_empty())
                    .map(|tx| {
                        let sent = tx
                            .input
                            .iter()
                            .map(|txin| {
                                let txid = txin.previous_output.txid;

                                let vout = txin.previous_output.vout;

                                self.outpoint_to_value(txid, vout, db, caches)
                            })
                            .sum::<u64>();

                        let recieved = tx.output.iter().map(|txout| txout.value).sum::<u64>();

                        sent - recieved
                    })
            })
            .sum::<u64>()
    }
}
