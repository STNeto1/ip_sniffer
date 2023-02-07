use std::{
    io::{self, Write},
    net::IpAddr,
    str::FromStr,
    sync::mpsc::{channel, Sender},
    thread,
};

use bpaf::Bpaf;
use tokio::net::TcpStream;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Args {
    #[bpaf(long, short)]
    pub address: IpAddr,
    #[bpaf(long("start"), short('s'), fallback(1u16))]
    pub start_port: u16,
    #[bpaf(long("end"), short('e'), fallback(MAX))]
    pub end_port: u16,
}

impl Args {}

const MAX: u16 = 65535;

#[tokio::main]
async fn main() {
    let opts = args().run();

    let (tx, rx) = channel::<u16>();
    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();

        tokio::spawn(async move {
            scan(tx, i, opts.address).await;
        });
    }

    drop(tx);
    let mut out: Vec<u16> = rx.into_iter().collect();
    out.sort();
    for p in out {
        println!("port: {}", p);
    }
}

async fn scan(tx: Sender<u16>, port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        Err(_) => {}
    }
}
