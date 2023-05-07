use bitcoin_explorer::{BitcoinDB, FBlock};
use chrono::NaiveDate;
use std::marker::PhantomData;

use crate::{
    output::*,
    processors::{DBCaches, DailyBlocksProcessor, Processor},
};

pub struct TransactionsCounter;

pub type TransactionsCounterProcessor = Processor<usize, TransactionsCounter>;

impl TransactionsCounterProcessor {
    pub fn new(path: &str) -> Self {
        Self {
            name: "Counter/Transactions".to_string(),
            output: Output::new(path, "counters/transactions.json"),
            pd: PhantomData,
        }
    }
}

impl DailyBlocksProcessor<usize> for TransactionsCounterProcessor {
    fn process_daily_blocks(
        &self,
        blocks: &[FBlock],
        _: &BitcoinDB,
        _: &DBCaches,
        _: &NaiveDate,
    ) -> usize {
        blocks.iter().map(|block| block.txdata.len()).sum()
    }
}
