#[macro_use]
extern crate error_chain;

extern crate tcpbench;
use tcpbench::errors::*;
use tcpbench::*;

extern crate clap;
use clap::{Arg, App, SubCommand};

extern crate histo;
use histo::Histogram;

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
                    Arg::with_name("count")
                        .short("c")
                        .long("count")
                        .help("number of connections to make")
                        .value_name("TRIES")
                        .takes_value(true)
                        .default_value("10"),
                )
                .arg(
                    Arg::with_name("pings")
                        .short("p")
                        .long("pings")
                        .help("number of times to measure RTT per connection")
                        .value_name("PINGS")
                        .takes_value(true)
                        .default_value("5"),
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
            let tries: u32 = sub.value_of("count").unwrap().parse().unwrap();
            let address = sub.value_of("address").unwrap();
            let pings: u32 = sub.value_of("pings").unwrap().parse().unwrap();
            let mut r = Run::new(address.into(), tries, pings);
            println!("running {} probes to {}...", tries, address);
            let rr = r.run().unwrap();

            summarize(&rr);
            Ok(())
        }
        ("server", Some(sub)) => {
            let s = Server::listen(sub.value_of("address").unwrap().into())?;
            println!("serving forever");
            s.serve()
        }
        _ => Err(
            ErrorKind::ConfigError("must specify client or server".into()).into(),
        ),
    }
}


fn summarize(rs: &Vec<RunResult>) {
    let mut connect_hist = Histogram::with_buckets(10);
    let mut ping_hist = Histogram::with_buckets(10);

    for rr in rs {
        for pd in &rr.ping_durations {
            ping_hist.add(pd.num_microseconds().unwrap() as u64);
        }
        connect_hist.add(rr.connect_duration.num_microseconds().unwrap() as u64);
    }
    println!("# Connect duration in microseconds:");
    println!("{}", connect_hist);
    println!("# ---------------------------------");
    println!("# RTT in microseconds:");
    println!("{}", ping_hist);
}
