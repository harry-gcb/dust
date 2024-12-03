mod cli;
mod config;
mod progress;

use crate::cli::build_cli;
use crate::progress::RuntimeErrors;
use clap::parser::ValuesRef;
use std::fs::read_to_string;
use std::io;
use std::process;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::vec;

use clap::error;
use config::get_config;
use regex::Regex;
use std::cmp::max;
use terminal_size::{terminal_size, Height, Width};

static DEFAULT_NUMBER_OF_LINES: usize = 30;
static DEFAULT_TERMINAL_WIDTH: usize = 80;

fn get_height_of_terminal() -> usize {
    terminal_size()
        .map(|(_, Height(h))|max(h.into(), DEFAULT_NUMBER_OF_LINES))
        .unwrap_or(DEFAULT_NUMBER_OF_LINES) - 10
}

fn get_width_of_terminal() -> usize {
    terminal_size()
        .map(|(Width(w), _)| match cfg!(windows) {
            // Windows CI runners detect a very low terminal width
            true => max(w.into(), DEFAULT_TERMINAL_WIDTH),
            false => w.into(),
        })
        .unwrap_or(DEFAULT_TERMINAL_WIDTH)
}

fn get_regex_value(maybe_value: Option<ValuesRef<String>>) -> Vec<Regex> {
    maybe_value
        .unwrap_or_default()
        .map(|reg|{
            Regex::new(reg).unwrap_or_else(|err|{
                eprintln!("Ignoring bad value for regex {err:?}");
                process::exit(1);
            })
        })
        .collect()
}

fn main() {
    let options = build_cli().get_matches();
    let config = get_config();

    let errors = RuntimeErrors::default();
    let error_listen_for_ctrlc = Arc::new(Mutex::new(errors));
    let errors_for_rayon = error_listen_for_ctrlc.clone();
    let errors_final = error_listen_for_ctrlc.clone();
    let is_in_listing = Arc::new(AtomicBool::new(false));
    let cloned_is_in_listing = Arc::clone(&is_in_listing);

    ctrlc::set_handler(move || {
        error_listen_for_ctrlc.lock().unwrap().abort = true;
        println!("\nAborting");
        if cloned_is_in_listing.load(Ordering::Relaxed) {
            process::exit(1);
        }
    })
    .expect("Error setting Ctrl-C handler");

    is_in_listing.store(true, Ordering::Relaxed);
    let target_dirs = match config.get_files_from(&options) {
        Some(path) => {
            if path == "-" {
                let mut targets_to_add = io::stdin()
                    .lines()
                    .map_while(Result::ok)
                    .collect::<Vec<String>>();

                if targets_to_add.is_empty() {
                    eprintln!("No input provided, defaulting to current directory");
                    targets_to_add.push(".".to_owned());
                }
                targets_to_add
            } else {
                // read file
                match read_to_string(path) {
                    Ok(file_content) => file_content.lines().map(|x|x.to_string()).collect(),
                    Err(e) => {
                        eprintln!("Error reading file: {e}");
                        vec![".".to_owned()]
                    }
                }
            }
        },
        None => match options.get_many("params") {
            Some(values) => values.cloned().collect(),
            None => vec![".".to_owned()],
        },
    };
    is_in_listing.store(false, Ordering::Relaxed);

    let summarize_file_types = options.get_flag("types");

    let filter_regexs = get_regex_value(options.get_many("filter"));
    let invert_filter_regexs = get_regex_value(options.get_many("invert_filter"));

    let terminal_width = match options.get_one("width") {
        Some(&val) => val,
        None => get_width_of_terminal(),
    };

    let depth = config.get_depth(&options);

    // If depth is set, then we set the default number_of_lines to be max
    // instead of screen height

    let number_of_lines = match options.get_one("number_of_lines") {
        Some(&value) => value,
        None => {
            if depth != usize::MAX {
                usize::MAX
            } else {
                get_height_of_terminal()
            }
        }
    };

}