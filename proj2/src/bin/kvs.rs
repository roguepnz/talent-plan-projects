use clap::{load_yaml, App};
use std::env;
use std::process::exit;
use proj2::kvs::KvStore;

fn main() {
    let store = KvStore::open("sdad").unwrap();
    // let yaml = load_yaml!("cli.yml");
    // let app = App::from(yaml);
    // let matches = app.get_matches();
    //
    // if matches.is_present("version") {
    //     println!(env!("CARGO_PKG_VERSION"));
    // }
    //
    // match matches.subcommand() {
    //     Some(("get", _args)) => {
    //         error_exit("unimplemented");
    //     }
    //     Some(("set", _args)) => {
    //         error_exit("unimplemented");
    //     }
    //     Some(("rm", _args)) => {
    //         error_exit("unimplemented");
    //     }
    //     _ => {
    //         unreachable!();
    //     }
    // }
}

fn error_exit(msg: &str) {
    eprintln!("{}", msg);
    exit(1);
}
