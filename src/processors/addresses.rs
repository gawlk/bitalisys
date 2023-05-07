use bitcoin_explorer::{BitcoinDB, FBlock, STransaction};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, marker::PhantomData};

use crate::{
    output::*,
    processors::{DailyBlocksProcessor, Processor},
};

#[derive(Serialize, Deserialize)]
pub struct AddressesMovement {
    from: HashMap<String, u64>,
    to: HashMap<String, u64>,
}

pub type AddressesProcessor = Processor<AddressesMovement, AddressesMovement>;

impl AddressesProcessor {
    pub fn new(path: &str) -> Self {
        Self {
            name: "Addresses".to_string(),
            output: Output::new(path, "addresses.json"),
            pd: PhantomData,
        }
    }
}

impl DailyBlocksProcessor<AddressesMovement> for AddressesProcessor {
    ///
    /// Allows the computation of the:
    /// - Volume
    /// - Realized price
    /// - Coin days destroyed
    /// - HODL waves
    ///
    fn process_daily_blocks(&self, blocks: &[FBlock], db: &BitcoinDB) -> AddressesMovement {
        let mut movement = AddressesMovement {
            from: HashMap::new(),
            to: HashMap::new(),
        };

        blocks.iter().for_each(|block| {
            block.txdata.iter().for_each(|tx| {
                tx.input.iter().for_each(|txin| {
                    let txid = txin.previous_output.txid;

                    let vout = txin.previous_output.vout;

                    let txprev = db.get_transaction::<STransaction>(&txid).unwrap();

                    let txout = txprev.output.get(vout as usize).unwrap();

                    let addresses_length = txout.addresses.len();

                    // Why
                    if addresses_length != 1 {
                        panic!(
                            "AddressesProcessor error for tx id {} (found {} addresses",
                            txid, addresses_length
                        );
                    }

                    let address_string = txout.addresses.first().unwrap().to_string();

                    let value = txout.value;

                    let previous_value = movement.from.get(&address_string).unwrap_or(&0);

                    movement.from.insert(address_string, value + previous_value);
                });

                tx.output.iter().for_each(|txout| {
                    let addresses_length = txout.addresses.len();

                    // Why
                    if addresses_length != 1 {
                        panic!(
                            "AddressesProcessor error for tx id {} (found {} addresses",
                            tx.txid, addresses_length
                        );
                    }

                    let address_string = txout.addresses.first().unwrap().to_string();

                    let value = txout.value;

                    let previous_value = movement.to.get(&address_string).unwrap_or(&0);

                    movement.to.insert(address_string, value + previous_value);
                })
            })
        });

        movement
    }
}
