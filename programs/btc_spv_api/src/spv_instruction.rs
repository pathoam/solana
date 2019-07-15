//! Spv proof Verification Program

use serde_derive::{Deserialize, Serialize};

pub type BitcoinTxHash = [u8;32];

pub struct ProofEntry    = {
    // 32 byte merkle hashes
    pub hash: [u8;32];
    // side of the merkle tree
    pub side: bool;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct BlockHeader {
    // holds block header parameters
}

pub type MerkleProof = [ProofEntry];

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ClientRequestInfo {
    // bitcoin transaction hash
    pub txHash:        BitcoinTxHash;
    // confirmation count
    pub confirmations: u8;
    // fee paid for tx verification
    pub fee:           u64;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ProofSubmitInfo {
    //
    pub proof: MerkleProof;
    // merkle branch connecting txhash to block header merkle root
    pub
}



#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum SpvInstruction {
    // Verify bitcoin transaction
    ClientRequest(ClientRequestInfo);

    ProofSubmit()


}
