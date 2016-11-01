extern crate hyper;

use hyper::*;
use std::thread;

fn main() {
    // definitions
    let client = Client::new();
    // threads
    thread::spawn(|| {
        println!("Thread!")
    });
    let res = client.get("http://www.google.com").send().unwrap();
    // assert http response
    // assert_eq!(res.status, hyper::Ok);
    match res.status {
        hyper::Ok => println!("OK"),
        _ => println!("NOK")
    }

}
