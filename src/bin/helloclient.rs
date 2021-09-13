use std::env;
use std::str;
use std::time::Instant;
use std::str::FromStr;
use powddos::{ChallengeRequest,InitRequest, ChallengeResponse, Payload, MD5hash, PoWChecker};
use async_std::net::ToSocketAddrs;
use async_std::net::TcpStream;
use async_std::task;
use async_std::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn client<Checker:PoWChecker, AddrT: ToSocketAddrs>(addr: AddrT) -> Result<()> {
    const BUFFER_SIZE: usize = 1024;
    const MAXITERS: u32 = 100_000_000;
    let mut stream = TcpStream::connect(addr).await?;

    let mut buffer = [0; BUFFER_SIZE];

    let req = InitRequest {};
    stream.write_all(&serde_json::to_vec(&req).unwrap()).await?;

    let size = stream.read(&mut buffer).await?;
    let chreq: ChallengeRequest = serde_json::from_slice(&buffer[..size])?;
    
    //try get some val
    let start = Instant::now();

    let nonce = match Checker::getpropernonce(MAXITERS, chreq.base, chreq.threshold) {
        Ok(val) => val,
        Err(_err) => return Err("too heavy for me")?,
    };
    let elapsed = start.elapsed();
    println!("found good nonce for {} millis", elapsed.as_millis());
    
    let chresponse = ChallengeResponse { base: chreq.base, nonce: nonce };
    stream.write_all(&serde_json::to_vec(&chresponse).unwrap()).await?;
    

    // we don't know, how long is payload
    let mut buf = Vec::new();
    match stream.read_to_end(&mut buf).await {
        Ok(ret) => ret,
        Err(_err) => return Err("too long :(")?,
    };
    let payload: Payload = match serde_json::from_slice(&buf[..]) {
        Ok(ret) => ret,
        Err(_err) => return Err("too long :(")?,
    };
    
    println!("payload:\n{}", payload.value);
    
    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let addr = match args.len() {
        1 => "0.0.0.0:4321",
        _ => &args[1],
    };
    let tries = match args.len() {
        1 | 2 => 10,
        _ => i32::from_str(&args[2]).unwrap_or(10),
    };

    for _iter in 1..tries {
        let fut = client::<MD5hash, &str>(addr);
        match task::block_on(fut) {
            Ok(ret) => ret,
            Err(err) => println!("err with request {}", err),
        };
    }
}
