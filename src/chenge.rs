use std::{env, fs::{self, File}, io::Write};

use crate::{collect, compress, labels::{Label, Labels}, paths::{build_dir, mg}, spinner, unpack};




pub fn chenge(label: Option<String>, recovery: bool) {

    let label_exists: Label;

    if let Some(p) = label {   
        label_exists = Labels::get_label(&p).unwrap_or_else(|| {
            eprintln!("\x1b[31mAbort:\x1b[0m Label not found");
            std::process::exit(1);
        });        
    } else {
        label_exists = Labels::get_latest_labels().unwrap_or_else(|e| {
            eprintln!("Error getting latest label: {}", e);
            std::process::exit(1);
        });
    }

    if !recovery{
        println!("Found label: \x1b[33m{}\x1b[0m", label_exists.name);
        println!("You really chenge the label to {}?", label_exists.name)
    }else{
        println!("Reset to \x1b[33mrecovery\x1b[0m label");    
        println!("You really reset the progress to recovery?");
    }

    print!("[y/n] ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if matches!(input.trim(), "y" | "yes") {
        chenge_real(label_exists, recovery);
    }    
}


fn chenge_real(label: Label, recovory: bool) {
    let mg_path = mg();
    let current = env::current_dir().unwrap();

    let recovery_path = mg_path.join("saves").join("recovery").join("save.tar.zst");

    if recovory && recovery_path.exists() {
        println!("\x1b[33mINFO:\x1b[0m Restoring progress from 'recovery'...");

        unpack::unpack(recovery_path.clone(), current.clone()).unwrap_or_else(|e| {
            eprintln!("\x1b[31mAbort:\x1b[0m Failed to restore 'recovery' label: {}", e);
            std::process::exit(1);
        });
        println!("\x1b[32mINFO:\x1b[0m Successfully restored progress from 'recovery' label.");
        std::process::exit(0);

    }else if recovory && !recovery_path.exists(){
        println!("\x1b[31mAbort:\x1b[0m label is not found");
        std::process::exit(1);
    }

    // Save current progress to "recovery" label
    println!("\x1b[33mINFO:\x1b[0m Saving current progress to 'recovery' label...");
    collect::collect_files().unwrap_or_else(|e| {
        eprintln!("\x1b[31mAbort:\x1b[0m Failed to collect files: {}", e);
        std::process::exit(1);
    });

    let tar_file = File::open(build_dir().join("archive.tar")).unwrap_or_else(|e| {
        eprintln!("\x1b[31mAbort:\x1b[0m Failed to open tar file: {}", e);
        std::process::exit(1);
    });

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

    let file_path = mg_path.join("saves").join(label.uuid).join("save.tar.zst");

    let prg = spinner::spawn_new_spinner();

    // Remove all directories/files in the current directory except ".mg"
    for dir in current.read_dir().unwrap() {
        let dir = dir.unwrap().path();
        let file_name = dir.file_name().unwrap().to_str().unwrap().to_string();

        prg.set_message(file_name.clone());

        if file_name != ".mg" {
            if dir.is_file(){
                fs::remove_file(&dir).unwrap_or_else(|e| {
                    eprintln!("\x1b[31mFailed:\x1b[0m Failed to remove directory: {}", e);
                    std::process::exit(1);
                });
                continue;
            }

            fs::remove_dir_all(dir).unwrap_or_else(|e| {
                eprintln!("\x1b[31mFailed:\x1b[0m Failed to remove directory: {}", e);
                std::process::exit(1);
            });
        }
    }

    prg.set_message("Unpacking files...");

    // Unpack the files from the tar file
    unpack::unpack(file_path, current).unwrap_or_else(|e| {
        eprintln!("\x1b[31mAbort:\x1b[0m Failed to reset files: {}", e);
        eprintln!("Auto recovery...");
        rescure();
        eprintln!("\x1b[32mok\x1b[0m");
        std::process::exit(1);
    });

    prg.finish_and_clear();

    println!("Reset: \x1b[32mok\x1b[0m");
    println!("\x1b[33mYou can use 'mg chenge -r' to recover files\x1b[0m");
}

fn rescure() {
    let recovery_path = mg().join("recovery").join("recovery.tar.xst");    
    let current = env::current_dir().unwrap();
    unpack::unpack(recovery_path, current).unwrap_or_else(|_| {
        eprintln!("\x1b[31mFailed to recovery files!\x1b[0m");
        std::process::exit(1);
    });
}