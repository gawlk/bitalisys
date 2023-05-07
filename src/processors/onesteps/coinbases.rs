use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::NaiveDate;
use std::marker::PhantomData;

use crate::{
    output::*,
    processors::{DBCaches, DailyBlocksProcessor, Processor},
};

pub struct CoinbasesCounter;

pub type CoinbasesCounterProcessor = Processor<u64, CoinbasesCounter>;

impl CoinbasesCounterProcessor {
    pub fn new(path: &str) -> Self {
        Self {
            name: "Counter/Coinbases".to_string(),
            output: Output::new(path, "counters/coinbases.json"),
            pd: PhantomData,
        }
    }
}

impl DailyBlocksProcessor<u64> for CoinbasesCounterProcessor {
    fn process_daily_blocks(
        &self,
        blocks: &[FBlock],
        _: &BitcoinDB,
        _: &DBCaches,
        _: &NaiveDate,
    ) -> u64 {
        blocks
            .iter()
            .flat_map(|block| {
                block
                    .txdata
                    .iter()
                    .filter(|tx| tx.input.is_empty())
                    .map(|tx| {
                        let recieved = tx.output.iter().map(|txout| txout.value).sum::<u64>();

                        recieved
                    })
            })
            .sum::<u64>()
    }
}
