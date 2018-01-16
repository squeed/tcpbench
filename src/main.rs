#[macro_use]
extern crate error_chain;

extern crate tcpbench;
use tcpbench::errors::*;
use tcpbench::*;

extern crate clap;
use clap::{Arg, App, SubCommand};

extern crate histo;
use histo::Histogram;
use std::iter::Iterator;

quick_main!(run);

fn run() -> Result<()> {
    let matches = App::new("tcpbench")
        .version("0.0.1")
        .author("Casey Callendrello <c1@caseyc.net>")
        .about("simple TCP performance benchmark")
        .subcommand(
            SubCommand::with_name("server").arg(
                Arg::with_name("address")
                    .help("address:port to listen on")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            SubCommand::with_name("client")
                .arg(
                    Arg::with_name("tries")
                        .short("t")
                        .long("tries")
                        .help("number of connections to make")
                        .value_name("TRIES")
                        .takes_value(true)
                        .default_value("10"),
                )
                .arg(
                    Arg::with_name("address")
                        .help("address:port to connect to")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("client", Some(sub)) => {
            let tries: u32 = sub.value_of("tries").unwrap().parse().unwrap();
            let mut r = Run::new(sub.value_of("address").unwrap().into(), tries, 2);
            let rr = r.run().unwrap();

            summarize(&rr);
            Ok(())
        }
        //("server", Some(sub)) => {}
        _ => Err(
            ErrorKind::ConfigError("must specify client or server".into()).into(),
        ),
    }
}


fn print_result(rr: &RunResult) {
    println!(
        "connect took: {:?} ns",
        rr.connect_duration.num_microseconds().unwrap()
    )
}


fn summarize(rs: &Vec<RunResult>) {
    let mut connect_hist = Histogram::with_buckets(10);
    let mut ping_hist = Histogram::with_buckets(10);

    for rr in rs {
        connect_hist.add(rr.connect_duration.num_microseconds().unwrap() as u64);
    }
    println!("{}", connect_hist);
    //println!("{}", ping_hist);
}
