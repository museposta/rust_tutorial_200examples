

extern crate pipeliner;
use pipeliner::Pipeline;
use std::net::ToSocketAddrs;

// use std::net::*;

fn main() {
    let addresses: Vec<_> = (1..40).map(|n| format!("192.168.0.{}:0",n)).collect();
    let n = addresses.len();

    for result in addresses.with_threads(n).map(|s| s.to_socket_addrs()) {
        println!("got: {:?}", result);
    }
}