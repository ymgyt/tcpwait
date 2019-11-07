use clap::{crate_authors, crate_version, App, Arg};

mod command;

fn main() {
    let m = App::new("tcpwait")
        .author(crate_authors!())
        .version(crate_version!())
        .about("wait tcp connection")
        .arg(
            Arg::with_name("TARGET")
                .help("host:port")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("interval")
                .long("interval")
                .help("check interval")
                .default_value("1"),
        )
        .arg(
            Arg::with_name("max_retry")
                .long("max-retry")
                .alias("max-retries")
                .default_value("10"),
        )
        .get_matches();

    let cmd = command::Builder::new()
        .target(m.value_of("TARGET").unwrap())
        .interval(m.value_of("interval").unwrap())
        .max_retry(m.value_of("max_retry").unwrap())
        .build();

    match cmd.run() {
        Ok(_) => println!("OK"),
        Err(e) => println!("{:?}", e),
    }
}
