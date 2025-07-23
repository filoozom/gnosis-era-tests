use alloy_consensus::{BlockBody, ReceiptWithBloom, Receipts};
use eyre::Result;
use gnosis_primitives::header::GnosisHeader;
use reth_era::{
    e2s_types::E2sError,
    era1_file::Era1Reader,
    execution_types::{BlockTuple, DecodeCompressed},
};
use reth_primitives::TransactionSigned;
use std::{error::Error, fs::File, path::Path};

fn decode_block<E>(
    block: Result<BlockTuple, E>,
) -> eyre::Result<(
    GnosisHeader,
    BlockBody<TransactionSigned, GnosisHeader>,
    Receipts<ReceiptWithBloom>,
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

fn main() -> Result<()> {
    let file = File::open(Path::new("gnosis-00000-ac7f28ba.era1"))?;
    let reader = Era1Reader::new(file);

    for block in reader.iter().map(decode_block) {
        let (header, body, _receipts) = block?;
        println!("Block number: {}", header.number);

        for transaction in body.transactions.iter() {
            println!("Transaction: {:?}", transaction.tx_hash());
        }
    }

    Ok(())
}
