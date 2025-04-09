use std::fs::{self, File, read_dir};
use std::io::{self, BufReader, Write};
use tar::Builder;
use zstd::Encoder;

use crate::paths::{find_mg, mg};
use crate::uuid::get_uuid;
use crate::{labels, spinner};

pub fn compress_file(label: &str) -> std::io::Result<()> {
    let build_dir = mg().join("saves").join("build");
    let out_dir = mg().join("saves").join(get_uuid(label));
    fs::create_dir_all(&out_dir)?;
    fs::create_dir_all(&build_dir)?;

    // Create the tar archive
    let tar_path = build_dir.join("archive.tar");
    let mut archive = Builder::new(File::create(&tar_path)?);

    // Get the ignore files
    let ignore_path: Vec<String> = crate::get_ignore::get_ignore_files();

    let fmg = find_mg();

    let dir = read_dir(&fmg)?;
    let entries: Vec<_> = dir.collect::<Result<_, _>>()?;
    let count = entries.len();
    io::stdout().flush().unwrap();

    let progressbar = spinner::spawn_new_spinner();
    progressbar.set_message("Collecting files...");

    for (current_count, entry) in entries.into_iter().enumerate() {
        progressbar.set_message(format!("Collecting files: {}/{}", current_count + 1, count));
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();

        // Ensure paths in the archive are relative
        let relative_path = path.strip_prefix(find_mg()).unwrap();

        if ignore_path.contains(&file_name.to_string()) {
            continue;
        }

        if path.is_file() {
            archive
                .append_path_with_name(&path, relative_path)
                .unwrap_or_else(|e| {
                    eprintln!("Failed: {}: {}", file_name, e);
                    std::process::exit(1);
                });
        } else if path.is_dir() && path.file_name().unwrap() != ".mg" {
            archive
                .append_dir_all(relative_path, &path)
                .unwrap_or_else(|e| {
                    eprintln!("Failed: {}: {}", file_name, e);
                    std::process::exit(1);
                });
        }
    }
    progressbar.finish_and_clear();
    println!("Collect: ok");

    archive.finish()?;

    // Compress the tar archive with xz
    let tar_file = File::open(&tar_path)?;
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
