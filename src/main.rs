extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("SoftEELab")
        .version("0.1.0")
        .author("Marco Cosentino <cosentino.ma@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::with_name("config")
            .short("m")
            .long("model")
            .value_name("FILE")
            .help("The model to use")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .get_matches();
}
