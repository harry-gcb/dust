use clap::{value_parser, Arg, Command};

// For single thread mode set this variable on your command line:
// export RAYON_NUM_THREADS=1

pub fn build_cli() -> Command {
    Command::new("Dust")
        .about("Like du but more intuitive")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .value_name("DEPTH")
                .value_parser(value_parser!(usize))
                .help("Depth to show")
                .num_args(1)
        )
}