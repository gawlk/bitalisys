use bitcoin_explorer::{BitcoinDB, FBlock};
use itertools::Itertools;
use std::{path::Path, time::Instant};

mod output;
mod processors;
mod utils;

use processors::*;
use utils::*;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let timer = Instant::now();

    let path = Path::new("/Volumes/t7s/bitcoin");

    let db = BitcoinDB::new(path, true)?;

    let block_count = db.get_block_count();

    println!("\n{block_count} blocks found.");

    let path = "./jsons";

    let processors: Vec<Box<dyn DailyBlocksImporterPlusOutputExporter>> = vec![
        Box::new(BlocksCounterProcessor::new(path)),
        Box::new(DaysDestroyedCounterProcessor::new(path)),
        Box::new(InputsCounterProcessor::new(path)),
        Box::new(OutputsCounterProcessor::new(path)),
        Box::new(TransactionsCounterProcessor::new(path)),
        Box::new(FeesCounterProcessor::new(path)),
        Box::new(CoinbasesCounterProcessor::new(path)),
        Box::new(VolumeCounterProcessor::new(path)),
    ];

    let mut caches = DBCaches::new();

    // Don't change max or last data saved will be inacurate
    db.iter_block::<FBlock>(0, block_count)
        .batching(create_group_blocks_by_day_closure())
        .for_each(|blocks| {
            let first = blocks.first().unwrap();

            let date = timestamp_to_naive_date(first.header.time);

            println!("\n{date}...");

            processors.iter().for_each(|processor| {
                processor
                    .import_daily_blocks(date, &blocks, &db, &caches)
                    .unwrap()
            });

            caches.clear();
        });

    // processors
    //     .iter()
    //     .for_each(|processor| processor.export_output().unwrap());

    println!("Done in {} seconds", timer.elapsed().as_secs_f32());

    Ok(())
}
