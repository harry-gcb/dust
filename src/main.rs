mod cli;
mod config;
mod progress;

use crate::cli::build_cli;
use crate::progress::RuntimeErrors;
use std::io;
use std::process;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

use clap::error;
use config::get_config;

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

            }
        },
        None => todo!()
    };
}