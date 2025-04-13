use std::fs::{File, read_dir};
use std::io::{self, Write};
use tar::Builder;
use crate::paths::{build_dir, find_mg};
use crate::spinner;




pub fn collect_files() -> std::io::Result<Builder<File>> {
    let build_dir = build_dir();


    // Create the tar archive
    let tar_path = build_dir.join("archive.tar");
    let mut archive = Builder::new(File::create(&tar_path)?);

    // Get the ignore files
    let ignore_path: Vec<String> = crate::get_ignore::get_ignore_files();

    let fmg = find_mg();

    // read the directory
    let dir = read_dir(&fmg)?;
    let entries: Vec<_> = dir.collect::<Result<_, _>>()?;
    // count the entries
    let count = entries.len();
    io::stdout().flush().unwrap();

    let progressbar = spinner::spawn_new_spinner();
    progressbar.set_message("Collecting files...");

    // collect the files
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

    archive.finish()?;
    progressbar.finish_and_clear();
    println!("Collect: ok");

    Ok(archive)
}