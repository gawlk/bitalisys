use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::NaiveDate;
use std::marker::PhantomData;

use crate::{
    output::*,
    processors::{DBCaches, DailyBlocksProcessor, Processor},
};

pub struct OutputsCounter;

pub type OutputsCounterProcessor = Processor<usize, OutputsCounter>;

impl OutputsCounterProcessor {
    pub fn new(path: &str) -> Self {
        Self {
            name: "Counter/Outputs".to_string(),
            output: Output::new(path, "counters/outputs.json"),
            pd: PhantomData,
        }
    }
}

impl DailyBlocksProcessor<usize> for OutputsCounterProcessor {
    fn process_daily_blocks(
        &self,
        blocks: &[FBlock],
        _: &BitcoinDB,
        _: &DBCaches,
        _: &NaiveDate,
    ) -> usize {
        blocks
            .iter()
            .flat_map(|block| block.txdata.iter().map(|tx| tx.output.len()))
            .sum::<usize>()
    }
}
