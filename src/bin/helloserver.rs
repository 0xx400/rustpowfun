#[macro_use]
extern crate lazy_static;

use std::env;
use powddos::{ChallengeRequest,InitRequest, ChallengeResponse, Payload, MD5hash, PoWChecker};
use async_std::net::ToSocketAddrs;
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::task;
use async_std::prelude::*;
use async_std::future::timeout;
use std::time::Duration;
use std::fs;
use rand::seq::SliceRandom;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

lazy_static! {
    static ref CONT_QUOTES: String = {
        const FNAME: &str = "quotes.txt";
        let contents = match fs::read_to_string(FNAME) {
            Ok(ret) => ret,
            Err(_err) => String::from("Far far away\n\n----\nFar far away2\n\n"),
        };
        contents
    };
    static ref QUOTES: Vec::<&'static str> = {
        let quotes: Vec<&'static str> = CONT_QUOTES.split("----\n").collect();
        quotes
    };
}


fn payload() -> &'static str {
    match QUOTES.choose(&mut rand::thread_rng()) {
        Some(ret) => ret,
        None => "no quotes today",
    }
}

async fn handle_new_connection<Checker:PoWChecker>(mut stream: TcpStream) -> Result<()>{
    const BUFFER_SIZE: usize = 1024;
    const THRESHOLD: u32 = 17;

    let mut buffer = [0; BUFFER_SIZE];

    let size = match stream.read(&mut buffer).await {
        Ok(ret) => ret,
        Err(_error) => return Err("No first init")?,
    };

    let _req: InitRequest = match serde_json::from_slice(&buffer[..size]) {
        Ok(ret) => ret,
        Err(_error) => return Err("bad init json")?,
    };
    
    let base = Checker::get_base();
    let req = ChallengeRequest { base: base, threshold: THRESHOLD };
    stream.write_all(&serde_json::to_vec(&req).unwrap()).await?;

    let size = match stream.read(&mut buffer).await {
        Ok(ret) => ret,
        Err(_error) => return Err("No challenge response")?,
    };

    let chres: ChallengeResponse = match serde_json::from_slice(&buffer[..size]) {
        Ok(ret) => ret,
        Err(_error) => return Err("Bad json challenge response")?,
    };
    let _powcheck = Checker::checkhash(req.base, chres.nonce, req.threshold);

    let payload = Payload{ value: String::from(payload())};
    stream.write_all(&serde_json::to_vec(&payload).unwrap()).await?;

    Ok(())
}

async fn debug_handle_new_connection<Checker:PoWChecker>(stream: TcpStream) -> Result<()>{
    let ret = match handle_new_connection::<Checker>(stream).await {
        Ok(ret) => ret,
        Err(error) => println!("err while handling connection: {}", error),
    };
    Ok(ret)
}

async fn server<Checker: 'static + PoWChecker, AddrType: ToSocketAddrs>  (addr: AddrType) -> Result<()> {
    const TIMEOUT_MS: u64 = 1000;

    let listener = TcpListener::bind(addr).await?;
    let mut incoming = listener.incoming();
    
    while let Some(stream) = incoming.next().await {
        let stream = match stream {
            Ok(ret) => ret,
            Err(_error) => continue,
        };
        let _hand = task::spawn(timeout(
                    Duration::from_millis(TIMEOUT_MS), 
                    debug_handle_new_connection::<Checker>(stream))
                );
    }
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let addr = match args.len() {
        1 => "0.0.0.0:4321",
        _ => &args[1],
    };

    let fut = server::<MD5hash,&str>(addr);
    task::block_on(fut).unwrap();
}
