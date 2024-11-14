use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::crypto::{PublicKey, Signature};
use crate::sha256::Hash;
use crate::util::Saveable;
use std::io::{
  Error as IoError, ErrorKind as IoErrorKind, Read,
  Result as IoResult, Write,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction { inputs, outputs }
    }
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {
    pub prev_transaction_output_hash: Hash,
    pub signature: Signature, // dummy types, will be replaced later
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid,
    pub pubkey: PublicKey, // dummy types, will be replaced later
}

impl TransactionOutput {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

// save and load expecting CBOR from ciborium as format
impl Saveable for Transaction {
  fn load<I: Read>(reader: I) -> IoResult<Self> {
    ciborium::de::from_reader(reader).map_err(|_| {
      IoError::new(
        IoErrorKind::InvalidData,
        "Failed to deserialize Transaction",
      )
    })
  }
  fn save<O: Write>(&self, writer: O) -> IoResult<()> {
    ciborium::ser::into_writer(self, writer).map_err(
        |_| {
          IoError::new(
          IoErrorKind::InvalidData,
          "Failed to serialize Transaction",
        )
      },
    )
  }
}