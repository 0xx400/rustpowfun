use byteorder::{BigEndian, WriteBytesExt};
use std::mem;
use serde::{Deserialize, Serialize};
use rand::Rng;
use byteorder::ByteOrder;


pub trait PoWChecker {
    fn new() -> Self;
    fn get_base() -> u64;
    fn calchash(nonce: u64, value: u64) -> u64;
    fn checkhash(nonce: u64, value: u64, threshold: u32) -> bool;
    fn getpropernonce(maxiter: u32, base: u64, threshold: u32) -> Result<u64, &'static str>;
}

pub struct MD5hash {
}

impl PoWChecker for MD5hash  {
    fn new() -> Self {
        MD5hash {}
    }
    fn get_base() -> u64 {
        let mut rng = rand::thread_rng();
        let val: u64 = rng.gen();
        val
    }

    fn calchash(base: u64, nonce: u64) -> u64 {
        let mut bs = [0u8; 2 * mem::size_of::<u64>()];
        bs[..mem::size_of::<i64>()].as_mut().write_u64::<BigEndian>(base).expect("");
        bs[mem::size_of::<i64>()..].as_mut().write_u64::<BigEndian>(nonce).expect("");

        let digest = md5::compute(bs);
        let lastbytes = BigEndian::read_u64(&digest[..mem::size_of::<u64>()]);
        lastbytes
    }

    fn checkhash(base: u64, nonce: u64, threshold: u32) -> bool {
        let lastbytes = Self::calchash(base, nonce);
        let mask = (1 << threshold) - 1;
        return (lastbytes & mask) == 0;
    }
    fn getpropernonce(maxiter: u32, base: u64, threshold: u32) -> Result<u64, &'static str> {
        for nonce in 1..maxiter {
            if Self::checkhash(base, nonce as u64, threshold) {
                return Ok(nonce as u64);
            }
        }
        Err("max iter exceeded")
    }
}


#[derive(Deserialize, Serialize, Debug)]
pub struct InitRequest {
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChallengeRequest {
    pub base: u64,
    pub threshold: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChallengeResponse {
    pub base: u64,
    pub nonce: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Payload {
    pub value: String,
}

