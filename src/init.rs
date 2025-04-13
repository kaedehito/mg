use std::{fs, path::PathBuf, process::Command};

use crate::compress;

pub fn init(label: Option<String>) {
    if label.is_none() {
        println!("\x1b[33mWARN:\x1b[0m No label provided. Save to 'latest' label.");
    }
    let mg = PathBuf::new().join(".mg");
    if !mg.exists() {
        fs::create_dir_all(&mg).unwrap_or_else(|e| {
            eprintln!("\x1b[31mAbort:\x1b[0m Failed to create directory: {e}");
            std::process::exit(1);
        });

        // if windows. hides the directory
        if cfg!(target_os = "windows") {
            let _ = Command::new("cmd")
                .arg("/C")
                .arg(format!("attrib +h {}", &mg.display()))
                .status();
        }


        compress::compress_file(&label.clone().unwrap_or("latest".to_string())).unwrap_or_else(
            |e| {
                eprintln!("\x1b[31mAbort:\x1b[0m Failed to save files: {}", e);
                std::process::exit(1);
            },
        );
        println!(
            "\x1b[33mINFO:\x1b[0m Initialized a new progress with label: {}",
            label.unwrap_or("latest".to_string())
        );
    }else{
        eprintln!(
            "\x1b[31mAbort:\x1b[0m mg contorl is already initialized.",
        );
        std::process::exit(1);
    }
}
