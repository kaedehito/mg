use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use std::path::Path;
use zstd::Encoder;

use crate::paths::mg;
use crate::uuid::get_uuid;
use crate::{collect, labels, spinner};

pub fn compress_file(label: &str) -> std::io::Result<()> {
    let build_dir = mg().join("saves").join("build");
    let out_dir = mg().join("saves").join(get_uuid(label));

    if !build_dir.exists() {
        fs::create_dir_all(&build_dir)?;
    }

    if !out_dir.exists() {
        fs::create_dir_all(&out_dir)?;
    }    

    // Create the tar archive
    let tar_path = build_dir.join("archive.tar");

    collect::collect_files()?;

    let tar_file = File::open(&tar_path)?;

    compress(tar_file, out_dir)?;

    let is_new_label = labels::Labels::find_label(label);

    if is_new_label {
        println!("Saved to label '{}'", label);
    } else {
        println!("Saved to new label '{}'", label);
    }
    labels::Labels::add_label(label)?;
    fs::remove_dir_all(tar_path.parent().unwrap())?;

    Ok(())
}


pub fn compress<P: AsRef<Path>>(tar_file: File, out_dir: P) -> std::io::Result<()> {
    let out_dir = out_dir.as_ref();


    let output_file = File::create(out_dir.join("save.tar.zst"))?;

    let mut encoder = Encoder::new(output_file, 3).unwrap_or_else(|e| {
        eprintln!("Failed to create encoder: {}", e);
        std::process::exit(1);
    });

    let progressbar = spinner::spawn_new_spinner();
    progressbar.set_message("Compressing files...");

    io::stdout().flush().unwrap();
    std::io::copy(&mut BufReader::new(tar_file), &mut encoder)?;

    encoder.finish()?;
    progressbar.finish_and_clear();
    println!("Compress: ok");

    Ok(())
}

