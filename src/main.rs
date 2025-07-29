use crate::rpc_log::RpcLog;
use alloy_consensus::{BlockBody, ReceiptWithBloom};
use alloy_primitives::Log;
use eyre::Result;
use gnosis_primitives::header::GnosisHeader;
use reth_era::{
    e2s_types::E2sError,
    era1_file::Era1Reader,
    execution_types::BlockTuple,
    DecodeCompressed
};
use reth_primitives::TransactionSigned;
use std::{error::Error, fs::File, path::Path};

mod rpc_log;

fn decode_block<E>(
    block: Result<BlockTuple, E>,
) -> eyre::Result<(
    GnosisHeader,
    BlockBody<TransactionSigned, GnosisHeader>,
    Vec<ReceiptWithBloom>,
)>
where
    E: From<E2sError> + Error + Send + Sync + 'static,
{
    let block = block?;

    let header = block.header.decode()?;
    let body = block.body.decode()?;
    let receipts = block.receipts.decode()?;

    Ok((header, body, receipts))
}

fn get_topic(log: &Log, index: usize) -> Vec<u8> {
    log.data
        .topics()
        .get(index)
        .map_or_else(|| vec![], |topic| topic.to_vec())
}

fn main() -> Result<()> {
    let file = File::open(Path::new("gnosis-00000-ac7f28ba.era1"))?;
    let reader = Era1Reader::new(file);

    for block in reader.iter().map(decode_block) {
        let (header, body, receipts) = block?;
        let block_hash = header.hash_slow();

        for (transaction_index, receipt) in receipts.iter().enumerate() {
            let transaction_hash = body.transactions[transaction_index].tx_hash();

            for (log_index, log) in receipt.receipt.logs.iter().enumerate() {
                let rpc_log = RpcLog {
                    address: log.address.to_vec(),
                    topic_0: get_topic(log, 0),
                    topic_1: get_topic(log, 1),
                    topic_2: get_topic(log, 2),
                    topic_3: get_topic(log, 3),
                    data: log.data.data.to_vec(),
                    block_number: header.number,
                    transaction_hash: transaction_hash.to_vec(),
                    transaction_index: transaction_index as u64,
                    block_hash: block_hash.to_vec(),
                    log_index: log_index as u64,
                };

                println!("{}", rpc_log);
            }
        }
    }

    Ok(())
}
