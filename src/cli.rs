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
        .arg(
            Arg::new("params")
                .value_name("PATH")
                .value_hint(clap::ValueHint::AnyPath)
                .value_parser(value_parser!(String))
                .num_args(1..)
        )
        .arg(
            Arg::new("files0_from")
                .long("files0-from")
                .value_hint(clap::ValueHint::AnyPath)
                .value_parser(value_parser!(String))
                .num_args(1)
                .help("run dust on NUL-terminated file names specified in file; if argument is -, then read names from standard input")
        )
        .arg(
            Arg::new("invert_filter")
                .short('v')
                .long("invert-filter")
                .value_name("REGEX")
                .action(clap::ArgAction::Append)
                .conflicts_with("filter")
                .conflicts_with("types")
                .help("Exclude filepaths matching this regex. To ignore png files type: -v \"\\.png$\" "),
        )
        .arg(
            Arg::new("filter")
                .short('e')
                .long("filter")
                .value_name("REGEX")
                .action(clap::ArgAction::Append)
                .conflicts_with("types")
                .help("Only include filepaths matching this regex. For png files type: -e \"\\.png$\" ")
        )
        .arg(
            Arg::new("types")
                .short('t')
                .long("file_types")
                .conflicts_with("depth")
                .conflicts_with("only_dir")
                .action(clap::ArgAction::SetTrue)
                .help("show only these file types")
        )
        .arg(
            Arg::new("only_dir")
                .short('D')
                .long("only-dir")
                .conflicts_with("only_file")
                .conflicts_with("types")
                .action(clap::ArgAction::SetTrue)
                .help("Only directories will be displayed.")
        )
        .arg(
            Arg::new("only_file")
                .short('F')
                .long("only-file")
                .conflicts_with("only_dir")
                .action(clap::ArgAction::SetTrue)
                .help("Only files will be displayed. (Finds your largest files)")
        )
}