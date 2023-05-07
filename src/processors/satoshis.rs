use bitcoin_explorer::{BitcoinDB, FBlock, SBlock, STransaction};
use std::{collections::HashMap, marker::PhantomData};

use crate::{
    output::*,
    processors::{DailyBlocksProcessor, Processor},
    utils::timestamp_to_naive_date,
};

type MovedSats = HashMap<String, u64>;
pub struct MovedSatsPhantomData;
pub type MovedSatsProcessor = Processor<MovedSats, MovedSatsPhantomData>;

impl MovedSatsProcessor {
    pub fn new(path: &str) -> Self {
        Self {
            name: "Moved sats".to_string(),
            output: Output::new(path, "satoshis.json"),
            pd: PhantomData,
        }
    }
}

impl DailyBlocksProcessor<MovedSats> for MovedSatsProcessor {
    ///
    /// Allows the computation of the:
    /// - Volume
    /// - Realized price
    /// - Coin days destroyed
    /// - HODL waves
    ///
    fn process_daily_blocks(&self, blocks: &[FBlock], db: &BitcoinDB) -> MovedSats {
        let mut map: MovedSats = HashMap::new();

        blocks.iter().for_each(|block| {
            block.txdata.iter().for_each(|tx| {
                tx.input.iter().for_each(|txin| {
                    let txid = txin.previous_output.txid;

                    let txprev = db.get_transaction::<STransaction>(&txid).unwrap();

                    let height = db.get_height_of_transaction(&txid).unwrap();

                    let block = db.get_block::<SBlock>(height).unwrap();

                    let date_string = timestamp_to_naive_date(block.header.time).to_string();

                    let vout = txin.previous_output.vout;

                    let txout = txprev.output.get(vout as usize).unwrap();

                    let value = txout.value;

                    let previous_value = map.get(&date_string).unwrap_or(&0);

                    map.insert(date_string, value + previous_value);
                })
            })
        });

        map
    }
}
