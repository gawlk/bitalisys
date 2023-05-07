use bitcoin_explorer::{BitcoinDB, FBlock};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::{
    output::*,
    processors::{DailyBlocksProcessor, Processor},
};

#[derive(Serialize, Deserialize)]
pub struct Counter {
    blocks: usize,
    transactions: usize,
    inputs: usize,
    outputs: usize,
}

pub type CounterProcessor = Processor<Counter, Counter>;

impl CounterProcessor {
    pub fn new(path: &str) -> Self {
        Self {
            name: "TQP".to_string(),
            output: Output::new(path, "counts.json"),
            pd: PhantomData,
        }
    }
}

impl DailyBlocksProcessor<Counter> for CounterProcessor {
    fn process_daily_blocks(&self, blocks: &[FBlock], _: &BitcoinDB) -> Counter {
        let mut counter = Counter {
            blocks: blocks.len(),
            transactions: 0,
            inputs: 0,
            outputs: 0,
        };

        blocks.iter().for_each(|block| {
            counter.transactions += block.txdata.len();

            block.txdata.iter().for_each(|tx| {
                counter.inputs += tx.input.len();
                counter.outputs += tx.output.len();
            })
        });

        counter
    }
}
