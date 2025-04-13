use std::{fs::{self, File}, path::Path};
use crate::{
    collect,
    compress,
    paths::build_dir,
};


pub fn recovery_label<P: AsRef<Path>>(mg_path: P) {
    let mg_path = mg_path.as_ref(); 

    // save to tar files
    collect::collect_files().unwrap_or_else(|e| {
        eprintln!("\x1b[31mAbort:\x1b[0m Failed to collect files: {}", e);
        std::process::exit(1);
    });

    let tar_file = File::open(build_dir().join("archive.tar")).unwrap_or_else(|e| {
        eprintln!("\x1b[31mAbort:\x1b[0m Failed to open tar file: {}", e);
        std::process::exit(1);
    });

    // compress tar file         

    let recovery_dir = mg_path.join("saves").join("recovery");
    if !recovery_dir.exists() {
        fs::create_dir_all(&recovery_dir).unwrap_or_else(|e| {
            eprintln!("\x1b[31mAbort:\x1b[0m Failed to create recovery directory: {}", e);
            std::process::exit(1);
        });
    }

    compress::compress(tar_file, recovery_dir).unwrap_or_else(|e| {
        eprintln!("\x1b[31mAbort:\x1b[0m Failed to save recovery label: {}", e);
        std::process::exit(1);
    });

    println!("Recovery files: \x1b[32mok\x1b[0m");
}

