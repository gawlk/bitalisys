use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::NaiveDate;
use std::marker::PhantomData;

use crate::{
    output::*,
    processors::{DBCaches, DailyBlocksProcessor, Processor},
};

pub struct InputsCounter;

pub type InputsCounterProcessor = Processor<usize, InputsCounter>;

impl InputsCounterProcessor {
    pub fn new(path: &str) -> Self {
        Self {
            name: "Counter/Inputs".to_string(),
            output: Output::new(path, "counters/inputs.json"),
            pd: PhantomData,
        }
    }
}

impl DailyBlocksProcessor<usize> for InputsCounterProcessor {
    fn process_daily_blocks(
        &self,
        blocks: &[FBlock],
        _: &BitcoinDB,
        _: &DBCaches,
        _: &NaiveDate,
    ) -> usize {
        blocks
            .iter()
            .flat_map(|block| block.txdata.iter().map(|tx| tx.input.len()))
            .sum::<usize>()
    }
}
