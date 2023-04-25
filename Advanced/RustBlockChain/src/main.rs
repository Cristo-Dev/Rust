use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, RwLock};
use std::io::{self, Write}; // Import io module for user input and output
use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use hex;

// Define the Block struct
#[derive(Clone)]
struct Block {
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

// Implement the Block struct
impl Block {
    // Calculate the hash of the block
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let timestamp_bytes = self.timestamp.to_be_bytes();
        let nonce_bytes = self.nonce.to_be_bytes();
        hasher.update(timestamp_bytes);
        hasher.update(self.data.as_bytes());
        hasher.update(self.previous_hash.as_bytes());
        hasher.update(nonce_bytes);
        let hash_result = hasher.finalize();
        hex::encode(&hash_result)
    }
}

// Define the Blockchain struct
struct Blockchain {
    blocks: Arc<RwLock<Vec<Block>>>,
}

// Implement the Blockchain struct
impl Blockchain {
    // Create a new Blockchain instance
    fn new() -> Blockchain {
        let genesis_block = Block {
            timestamp: 0,
            data: String::from("Genesis block"),
            previous_hash: String::from(""),
            hash: String::from(""),
            nonce: 0,
        };
        let mut blocks = Vec::new();
        blocks.push(genesis_block);
        Blockchain {
            blocks: Arc::new(RwLock::new(blocks)),
        }
    }

    // Get the latest block in the blockchain
    fn get_latest_block(&self) -> Block {
        let blocks = self.blocks.read().unwrap();
        blocks[blocks.len() - 1].clone()
    }

    // Add a new block to the blockchain
    fn add_block(&mut self, data: String) -> Block {
        let previous_block = self.get_latest_block();
        let mut new_block = Block {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            data: data,
            previous_hash: previous_block.hash.clone(),
            hash: String::from(""),
            nonce: 0,
        };
        let difficulty = 3;
        let mut hash = new_block.calculate_hash();
        while !hash.starts_with(&"0".repeat(difficulty)) {
            new_block.nonce += 1;
            hash = new_block.calculate_hash();
        }
        new_block.hash = hash;
        let mut blocks = self.blocks.write().unwrap();
        blocks.push(new_block.clone());
        new_block
    }

    // Check if the blockchain is valid
    fn is_valid(&self) -> bool {
        let blocks = self.blocks.read().unwrap();
        for (i, block) in blocks.iter().enumerate() {
            let hash = block.calculate_hash();
            if block.hash != hash {
                return false;
            }
            if i > 0 {
                let previous_block = &blocks[i - 1];
                if block.previous_hash != previous_block.hash {
                    return false;
                }
            }
        }
        true
    }
}

    
    fn main() {
        let mut blockchain = Blockchain::new();
        let mut input = String::new();
        
        loop {
            // Print menu
            println!("Choose an option:");
            println!("1. Add block");
            println!("2. Print blockchain");
            println!("3. Verify blockchain");
            println!("4. Exit");
    
            // Read user input
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            
            // Parse user input as integer
            let option = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            
            // Handle user input
            match option {
                1 => {
                    println!("Enter block data:");
                    let mut data = String::new();
                    io::stdin()
                        .read_line(&mut data)
                        .expect("Failed to read line");
                    blockchain.add_block(data.trim().to_string());
                },
                2 => {
                    println!("Blockchain:");
                    let blocks = blockchain.blocks.read().unwrap();
                    for block in blocks.iter() {
                        println!("Hash: {}", block.hash);
                        println!("Previous hash: {}", block.previous_hash);
                        println!("Data: {}", block.data);
                        println!("Timestamp: {}", block.timestamp);
                        println!("Nonce: {}\n", block.nonce);
                    }
                },
                3 => {
                    if blockchain.is_valid() {
                        println!("Blockchain is valid.");
                    } else {
                        println!("Blockchain is not valid.");
                    }
                },
                4 => {
                    println!("Exiting.");
                    break;
                },
                _ => continue,
            }
            
            // Reset user input
            input = String::new();
        }
    }
    