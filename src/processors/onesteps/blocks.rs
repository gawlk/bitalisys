use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::NaiveDate;
use std::marker::PhantomData;

use crate::{
    output::*,
    processors::{DBCaches, DailyBlocksProcessor, Processor},
};

pub struct BlocksCounter;

pub type BlocksCounterProcessor = Processor<usize, BlocksCounter>;

impl BlocksCounterProcessor {
    pub fn new(path: &str) -> Self {
        Self {
            name: "Counter/Blocks".to_string(),
            output: Output::new(path, "counters/blocks.json"),
            pd: PhantomData,
        }
    }
}

impl DailyBlocksProcessor<usize> for BlocksCounterProcessor {
    fn process_daily_blocks(
        &self,
        blocks: &[FBlock],
        _: &BitcoinDB,
        _: &DBCaches,
        _: &NaiveDate,
    ) -> usize {
        blocks.len()
    }
}
