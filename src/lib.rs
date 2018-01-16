#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate time;

pub mod errors {
    error_chain!{
        foreign_links {
            IO(::std::io::Error);
        }
        errors {
            NotImplemented(name: String){
                description("not implemented")
                display("not implemented: {}", name)
            }
            ConfigError(desc: String){
                description("bad configuration")
                display("bad config: {}", desc)
            }
        }
    }
}

use errors::*;
use std::io::prelude::*;
use std::net::TcpStream;
use time::{Duration, PreciseTime};
use std::thread;


pub struct Server {}
pub struct ServerConn {}

impl Server {
    // returns a Client
    pub fn accept(&self) -> Result<ServerConn> {
        Err(ErrorKind::NotImplemented("accept".into()).into())
    }
}

impl ServerConn {
    pub fn handle() {}
}

// Struct that tracks a test run
pub struct Run {
    address: String,
    tries: u32, // the number of times to do the run
    pings: u32, // The number of pings to do per connection
}

pub struct RunResult {
    pub connect_duration: Duration,
    pub ping_durations: Vec<Duration>,
    pub failed_pings: usize,
}

impl Run {
    pub fn new(address: String, tries: u32, pings: u32) -> Run {
        Run {
            address: address,
            tries: tries,
            pings: pings,
        }
    }

    pub fn run(&mut self) -> Result<Vec<RunResult>> {
        let mut out = vec![];

        for _ in 0..self.tries {
            match self.run_once() {
                Ok(rr) => out.push(rr),
                Err(_) => println!("error?"),
            }
            std::thread::sleep(std::time::Duration::from_millis(20));

        }

        return Ok(out);
    }

    pub fn run_once(&mut self) -> Result<RunResult> {
        let start = PreciseTime::now();
        let mut s = TcpStream::connect(self.address.clone())?;
        let end = PreciseTime::now();

        println!("connect ok!");
        let mut rr = RunResult {
            connect_duration: start.to(end),
            ping_durations: vec![],
            failed_pings: 0,
        };

        s.set_nodelay(true)?;

        for _ in 0..self.pings {
            match self.ping(&s) {
                Ok(d) => {
                    rr.ping_durations.push(d);
                    println!("Ping OK!");
                }
                Err(e) => rr.failed_pings += 1,
            };
        }

        Ok(rr)
    }

    fn ping(&self, mut s: &TcpStream) -> Result<Duration> {
        let t_0 = PreciseTime::now();
        s.write(&[10])?;
        let t_1 = PreciseTime::now();
        s.read(&mut [0; 1])?;
        let t_2 = PreciseTime::now();

        Ok(t_0.to(t_2))
    }
}
