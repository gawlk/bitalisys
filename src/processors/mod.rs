use std::{cell::RefCell, collections::HashMap, marker::PhantomData};

use bitcoin_explorer::{BitcoinDB, FBlock, SBlock, STransaction, Txid};
use chrono::{Datelike, NaiveDate};
use serde::{de::DeserializeOwned, Serialize};

use crate::{output::Output, utils::timestamp_to_naive_date};

// pub mod addresses;
// pub mod counter;
pub mod onesteps;
// pub mod satoshis;

// pub use addresses::*;
// pub use counter::*;
pub use onesteps::*;
// pub use satoshis::*;

pub struct Processor<T, P> {
    name: String,
    output: Output<T>,
    pd: PhantomData<P>,
}

pub struct DBCaches {
    pub txid_to_transaction: RefCell<HashMap<Txid, STransaction>>,
    pub txid_to_block: RefCell<HashMap<Txid, SBlock>>,
    pub txid_to_naive_date: RefCell<HashMap<Txid, NaiveDate>>,
    pub outpoint_to_value: RefCell<HashMap<(Txid, u32), u64>>,
}

impl DBCaches {
    pub fn new() -> Self {
        Self {
            txid_to_transaction: RefCell::new(HashMap::new()),
            txid_to_block: RefCell::new(HashMap::new()),
            txid_to_naive_date: RefCell::new(HashMap::new()),
            outpoint_to_value: RefCell::new(HashMap::new()),
        }
    }

    pub fn clear(&mut self) {
        self.txid_to_block.borrow_mut().clear();
        self.txid_to_block.borrow_mut().clear();
        self.txid_to_naive_date.borrow_mut().clear();
        self.outpoint_to_value.borrow_mut().clear();
    }
}

pub trait DailyBlocksProcessor<T> {
    fn process_daily_blocks(
        &self,
        blocks: &[FBlock],
        db: &BitcoinDB,
        caches: &DBCaches,
        date: &NaiveDate,
    ) -> T;

    fn txid_to_block(&self, txid: Txid, db: &BitcoinDB, caches: &DBCaches) -> SBlock {
        if let Some(block) = caches.txid_to_block.borrow().get(&txid) {
            return block.clone();
        }

        let height = db.get_height_of_transaction(&txid).unwrap();

        let block = db.get_block::<SBlock>(height).unwrap();

        caches
            .txid_to_block
            .borrow_mut()
            .insert(txid, block.clone());

        block
    }

    fn txid_to_naive_date(&self, txid: Txid, db: &BitcoinDB, caches: &DBCaches) -> NaiveDate {
        if let Some(date) = caches.txid_to_naive_date.borrow().get(&txid) {
            return date.to_owned();
        }

        let block = self.txid_to_block(txid, db, caches);

        let date = timestamp_to_naive_date(block.header.time);

        caches.txid_to_naive_date.borrow_mut().insert(txid, date);

        date
    }

    fn txid_to_tx(&self, txid: Txid, db: &BitcoinDB, caches: &DBCaches) -> STransaction {
        if let Some(transaction) = caches.txid_to_transaction.borrow().get(&txid) {
            return transaction.clone();
        }

        let transaction = db.get_transaction::<STransaction>(&txid).unwrap();

        caches
            .txid_to_transaction
            .borrow_mut()
            .insert(txid, transaction.clone());

        transaction
    }

    fn outpoint_to_value(&self, txid: Txid, vout: u32, db: &BitcoinDB, caches: &DBCaches) -> u64 {
        let outpoint = (txid, vout);

        if let Some(value) = caches.outpoint_to_value.borrow().get(&outpoint) {
            return *value;
        }

        let value = self
            .txid_to_tx(txid, db, caches)
            .output
            .get(usize::try_from(vout).unwrap())
            .unwrap()
            .value;

        caches
            .outpoint_to_value
            .borrow_mut()
            .insert(outpoint, value);

        value
    }
}

pub trait DailyBlocksImporter {
    fn import_daily_blocks(
        &self,
        date: NaiveDate,
        blocks: &[FBlock],
        db: &BitcoinDB,
        caches: &DBCaches,
    ) -> color_eyre::Result<()>;
}

impl<T, P> DailyBlocksImporter for Processor<T, P>
where
    T: DeserializeOwned + Serialize,
    Processor<T, P>: DailyBlocksProcessor<T>,
{
    fn import_daily_blocks(
        &self,
        date: NaiveDate,
        blocks: &[FBlock],
        db: &BitcoinDB,
        caches: &DBCaches,
    ) -> color_eyre::Result<()> {
        if self.output.data.borrow().get(&date.to_string()).is_none() {
            println!("Processing {}", self.name);

            self.output.data.borrow_mut().insert(
                date.to_string(),
                self.process_daily_blocks(blocks, db, caches, &date),
            );

            let day = date.day();

            if day == 1 || day == 14 {
                println!("Saving {}", self.name);

                self.export_output()?;
            }
        } else {
            println!("Skipping {}", self.name);
        }

        Ok(())
    }
}

pub trait OutputExporter {
    fn export_output(&self) -> color_eyre::Result<()>;
}

impl<T, P> OutputExporter for Processor<T, P>
where
    T: DeserializeOwned + Serialize,
{
    fn export_output(&self) -> color_eyre::Result<()> {
        self.output.export_json()?;
        Ok(())
    }
}

pub trait DailyBlocksImporterPlusOutputExporter: DailyBlocksImporter + OutputExporter {}

impl<T, P> DailyBlocksImporterPlusOutputExporter for Processor<T, P>
where
    T: DeserializeOwned + Serialize,
    Processor<T, P>: DailyBlocksProcessor<T>,
{
}
