use std::env;

use crate::{unpack, paths};



pub fn restore_recovery() {
    let Ok(current) = env::current_dir() else{
        eprintln!("\x1b[31mAbort:\x1b[0m Failed to get current directory.");
        std::process::exit(1);
    };

    let recovery_path = paths::mg().join("saves").join("recovery").join("recovery.tar.zst");    
    
    println!("\x1b[33mINFO:\x1b[0m Restoring progress from 'recovery'...");

    unpack::unpack(recovery_path.clone(), current.clone()).unwrap_or_else(|e| {
        eprintln!("\x1b[31mAbort:\x1b[0m Failed to restore 'recovery' label: {}", e);
        std::process::exit(1);
    });
    println!("\x1b[32mINFO:\x1b[0m Successfully restored progress from 'recovery' label.");
    std::process::exit(0);
}




