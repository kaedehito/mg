use std::{
    fs,
    io::{self, Write},
};

use crate::{labels, paths::mg, uuid::get_uuid};

pub fn remove(label: &str) {
    let label_is_enable = labels::Labels::find_label(label);

    if !label_is_enable {
        eprintln!("Label '{}' is not found", label);
        std::process::exit(1);
    }

    println!(
        r#"
Are you sure you want to delete the label '{}'?
This action will erase all progress saved under the label '{}'"#,
        &label, &label
    );
    println!("\x1b[33mThis action cannot be undone.\x1b[0m");

    print!("[y/n] ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if matches!(input.trim(), "y" | "yes") {
        fs::remove_dir_all(mg().join("saves").join(get_uuid(label))).unwrap_or_else(|e| {
            eprintln!("\x1b[31mAbort:\x1b[0m Failed to remove directory: {}", e);
        });
        labels::Labels::remove_label(label).unwrap_or_else(|e| {
            eprintln!("\x1b[31mAbort:\x1b[0m {}", e);
            std::process::exit(1);
        });
        println!("Label \"{}\" has been successfully deleted.", label);
        println!("All progress associated with this label has been erased.");
        println!("All related files and directories have also been removed.");

        return;
    } else {
        println!("remove canceled");
    }
}
