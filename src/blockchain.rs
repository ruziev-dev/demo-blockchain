extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate time;

use std::thread;
use std::time::Duration;

use indicatif::ProgressBar;
use serde_derive::Serialize;

use self::sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    sender: String,
    reciever: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct BlockHeader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: BlockHeader,
    count: u32,
    transaction: Vec<Transaction>,
}

#[derive(Debug, Serialize)]
pub struct Chain {
    chain: Vec<Block>,
    new_transactions: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            new_transactions: Vec::new(),
            difficulty,
            miner_addr,
            reward: 50.0,
        };
        chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, reciever: String, amount: f32) -> bool {
        let trans = Transaction {
            sender,
            reciever,
            amount,
        };
        self.new_transactions.push(trans);
        true
    }

    pub fn lash_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap(),
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fn update_reward(&mut self) -> bool {
        self.reward = self.reward / 2f32;
        println!("New reward: {}", self.reward);
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        let header = BlockHeader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            pre_hash: self.lash_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            reciever: self.miner_addr.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header,
            count: 0,
            transaction: vec![],
        };

        block.transaction.push(reward_trans);
        block.transaction.append(&mut self.new_transactions);
        block.count = block.transaction.len() as u32;

        block.header.merkle = Chain::get_merkle(block.transaction.clone());
        Chain::proof_of_work(&mut block.header);

        println!("last{:#?}", &block);
        self.chain.push(block);
        true
    }

    pub fn get_merkle(current_trans: Vec<Transaction>) -> String {
        let mut mercle = vec![];

        for t in &current_trans {
            let hash = Chain::hash(t);
            mercle.push(hash);
        }
        if mercle.len() % 2 == 1 {
            let last = mercle.last().cloned().unwrap();
            mercle.push(last);
        }

        while mercle.len() > 1 {
            let mut h1 = mercle.remove(0);
            let mut h2 = mercle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            mercle.push(nh);
        }
        mercle.pop().unwrap()
    }

    pub fn proof_of_work(header: &mut BlockHeader) {
        let difficulty = header.difficulty as u64;
        let pb = ProgressBar::new(1024);
        let d_progress = 8 / difficulty;
        let handlde = std::thread::spawn(move || {
            for _ in 0..(1024 / (d_progress)) {
                pb.inc(d_progress);
                thread::sleep(Duration::from_millis(difficulty * 10))
            }
            pb.finish_with_message("It has been done");
        });

        let mut m = String::from("");
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];

            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        m = hash;
                        break;
                    }
                }
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
        handlde.join().unwrap();
        println!("");
        println!("Block hash: {}", m);
        println!("");
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.input(input.as_bytes());
        let res = hasher.result();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        s
    }
}
