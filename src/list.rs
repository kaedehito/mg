pub fn list() {
    let mg_path = crate::paths::mg().join("saves").join("saves.json");

    if !mg_path.exists() {
        std::fs::create_dir_all(&mg_path.parent().unwrap()).unwrap_or_else(|e| {
            eprintln!("\x1b[31mAbort:\x1b[0m Failed to create directory: {e}");
            std::process::exit(1);
        });
        eprintln!("\x1b[31mAbort:\x1b[0m No saves.json file found. Please create a label first.");
        std::process::exit(1);
    }

    let file = std::fs::File::open(&mg_path).unwrap_or_else(|e| {
        eprintln!("\x1b[31mAbort:\x1b[0m Failed to open saves.json: {}", e);
        std::process::exit(1);
    });

    let reader = std::io::BufReader::new(file);

    let mut labels_data: crate::labels::Labels =
        serde_json::from_reader(reader).unwrap_or_else(|e| {
            eprintln!("\x1b[31mAbort:\x1b[0m Failed to Parse saves.json: {}", e);
            std::process::exit(1);
        });

    labels_data.labels.sort_by(|a, b| b.time.cmp(&a.time));

    for label in labels_data.labels {
        let time_str = &label.time;
        if let Ok(parsed_time) =
            chrono::DateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S%.f %:z")
        {
            let local_time = parsed_time.with_timezone(&chrono::Local);
            
            if label.current{
                println!(
                    "\x1b[33m{}\x1b[0m - Last saved at {}  \x1b[32m%\x1b[0m",
                    label.name,
                    local_time.format("%Y-%m-%d %H:%M:%S")
                );  
                continue;
            }

            println!(
                "\x1b[33m{}\x1b[0m - Last saved at {}",
                label.name,
                local_time.format("%Y-%m-%d %H:%M:%S")
            );
        } else {
            eprintln!(
                "\x1b[31mWarning:\x1b[0m Invalid timestamp for label '{}'.",
                label.name
            );
        }
    }
}
