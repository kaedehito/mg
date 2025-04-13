use indicatif::ProgressBar;
use std::fs;


pub fn remove_all(prg: ProgressBar){

    let current = std::env::current_dir().unwrap_or_else(|e| {
        eprintln!("\x1b[31mFailed:\x1b[0m Failed to get current directory: {}", e);
        std::process::exit(1);
    });


    
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
            }else{
                remove_dir(dir.clone(), prg.clone());
            }

        }
    }
}



fn remove_dir(dir: std::path::PathBuf, prg: ProgressBar) {
    if dir.is_dir() {
        let size = dir.read_dir().unwrap().count();


        for (current_len, entry) in dir.read_dir().unwrap().enumerate() {
            prg.set_message(format!("{} ({}/{})", dir.display(), current_len + 1, size));    
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                remove_dir(path, prg.clone());
            } else {
                fs::remove_file(path).unwrap_or_else(|e| {
                    eprintln!("\x1b[31mFailed:\x1b[0m Failed to remove file: {}", e);
                    std::process::exit(1);
                });
            }
        }


        fs::remove_dir(dir).unwrap_or_else(|e| {
            eprintln!("\x1b[31mFailed:\x1b[0m Failed to remove directory: {}", e);
            std::process::exit(1);
        });
    }
}