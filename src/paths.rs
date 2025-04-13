use std::path::PathBuf;

pub fn mg() -> PathBuf {
    let mut path = std::env::current_dir().expect("Failed to get current directory");
    loop {
        let candidate = path.join(".mg");
        if candidate.exists() && candidate.is_dir() {
            return candidate;
        } else if !candidate.is_dir() {
            eprintln!(
                "Abort: No .mg directory found in the current path or any parent directories."
            );
            std::process::exit(1);
        }

        if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        } else {
            break;
        }
    }
    eprintln!("Abort: No .mg directory found in the current path or any parent directories.");
    std::process::exit(1);
}

pub fn find_mgignore() -> Option<PathBuf> {
    let mut path = std::env::current_dir().expect("Failed to get current directory");
    let mg = find_mg();
    loop {
        let candidate = path.join(".mgignore");

        if candidate.exists() && candidate.is_file() {
            return Some(candidate);
        }

        if candidate.parent().unwrap() == mg {
            break;
        }

        if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        } else {
            break;
        }
    }
    None
}

pub fn build_dir() -> PathBuf {
    let mg = mg();
    let build_dir = mg.join("saves").join("build");
    if !build_dir.exists() {
        std::fs::create_dir_all(&build_dir).unwrap_or_else(|e| {
            eprintln!("Failed to create build directory: {}", e);
            std::process::exit(1);
        });
    }
    build_dir
}

pub fn find_mg() -> PathBuf {
    let mut path = std::env::current_dir().expect("Failed to get current directory");
    loop {
        let candidate = path.join(".mg");
        if candidate.exists() && candidate.is_dir() {
            return candidate.parent().unwrap().to_path_buf();
        } else if !candidate.is_dir() {
            eprintln!(
                "Abort: No .mg directory found in the current path or any parent directories."
            );
            std::process::exit(1);
        }

        if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        } else {
            break;
        }
    }
    eprintln!("Abort: No .mg directory found in the current path or any parent directories.");
    std::process::exit(1);
}
