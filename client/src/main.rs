#![allow(non_snake_case)]

use std::env;
use std::process;

use websocket::client::Url;
use websocket::ClientBuilder;
use websocket::r#async::Client;
use websocket::r#async::TcpStream;

fn usage() {

    println!("Usage: cargo run <server ip>");
    process::exit(-1);

}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
    }




    let cBuilder:Client<TcpStream> = ClientBuilder::new("ws://myapp.com")
    .unwrap()
    .connect_insecure()
    .unwrap();

    let url;
    
    match Url::parse(&args[1]) {
        Ok(val) => url=val,
        Err(_) => usage(),
    };




    println!("{:?}", args);
}
