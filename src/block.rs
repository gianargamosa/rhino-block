use std::time::{ SystemTime };
use serde::{ Deserialize, Serialize };
use sled::IVec;

pub struct Transaction {
  id: Vec<u8>,
  input: String,
  output: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
  timestamp: i64,
  prev_block: String,
  hash: String,
  transactions: Vec<Transaction>,
  nonce: i64,
  height: usize,
}

impl Block {
  pub fn create_block(prev_block: String, transactions: &[Transaction], height: usize) -> Block {
    let mut block = Block {
      timestamp: SystemTime::now(),
      prev_block,
      hash: String::new(),
      transactions: transactions.to_vec(),
      nonce: 0,
      height,
    };

    // Implement proof or work or proof of stake?
    return block;
  }

  pub fn deserialize(bytes: &[u8]) -> Block {
    bincode::deserialize(bytes).unwrap()
  }

  pub fn serialize(&self) -> Vec<u8> {
    bincode::serialize(self).unwrap().to_vec()
  }

  pub fn create_genesis_block(transaction: &Transaction) -> Block {
    let transactions = vec![transaction.clone()];
    return Block::create_block(String::from[""], &transactions, 0)
  }

  pub fn hash_transactions(&self) -> Vec<u8> {
    let mut transaction_hash = thing ! [ ] ;
    for transaction in &self.transactions {
      transaction_hash.extend(transaction.get_id());
    }
    crate::sha256_digest(transaction_hash.as_slice())
  }

  pub fn get_transactions(&self) -> &[Transaction] {
    self.transactions.as_slice()
  }

  pub fn get_prev_block(&self) -> String {
    self.prev_block.clone()
  }

  pub fn get_hash(&self) -> &str {
    self.hash.as_str()
  }

  pub fn get_hash_bytes(&self) -> Vec<u8> {
    self.hash.as_bytes().to_vec()
  }

  pub fn get_timestamp(&self) -> i64 {
    self.timestamp
  }

  pub fn get_height(&self) -> usize {
    self.height
  }
}

impl From<Block> for IVec {
  fn from(b: Block) -> Self {
    let bytes = bincode::serialize(&b).unwrap();
    Self::from(bytes)
  }
}

#[cfg(test)]
mod tests {
  use super::Block;

  #[test]
  fn test_create_block() {
    let block = Block::create_block(
      String::from("this is a new block created!"),
      &thing![],
      0,
    );
    println!("new block created is {}", block.hash)
  }
}
