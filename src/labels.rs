use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

use crate::uuid::get_uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Label {
    pub name: String,
    pub uuid: String,
    pub time: String, // Use chrono::DateTime
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Labels {
    pub labels: Vec<Label>,
}

fn read_and_labels() -> Labels {
    let mg_path = crate::paths::mg().join("saves").join("saves.json");

    if !mg_path.exists() {
        std::fs::create_dir_all(&mg_path.parent().unwrap()).unwrap_or_else(|e| {
            eprintln!("\x1b[31mAbort:\x1b[0m Failed to create directory: {e}");
            std::process::exit(1);
        });

        let file = std::fs::File::create(&mg_path).unwrap_or_else(|e| {
            eprintln!("\x1b[31mAbort:\x1b[0m Failed to create file: {e}");
            std::process::exit(1);
        });

        let towrite = Labels { labels: Vec::new() };
        serde_json::to_writer(file, &towrite).unwrap_or_else(|e| {
            eprintln!("\x1b[31mAbort:\x1b[0m Failed to wite saves.json: {e}");
            std::process::exit(1);
        });
    }
    let file = std::fs::File::open(&mg_path).unwrap_or_else(|e| {
        eprintln!("\x1b[31mAbort:\x1b[0m Failed to open saves.json: {}", e);
        std::process::exit(1);
    });

    let reader = std::io::BufReader::new(file);

    let labels_data: Labels = serde_json::from_reader(reader).unwrap_or_else(|e| {
        eprintln!("\x1b[31mAbort:\x1b[0m Failed to Parse saves.json: {}", e);
        std::process::exit(1);
    });

    labels_data
}

impl Labels {
    pub fn find_label(name: &str) -> bool {
        let labels_data = read_and_labels();

        let name_hash = get_uuid(name);

        let p = labels_data.labels.iter().find(|f| f.uuid == name_hash);

        p.is_some()
    }

    pub fn get_label(name: &str) -> Option<Label> {

        let labels_data = read_and_labels();

        let name_hash = get_uuid(name);

        labels_data.labels.into_iter().find(|f| f.uuid == name_hash)
    }

    
    pub fn add_label(name: &str) -> std::io::Result<()> {
        let mg_path = crate::paths::mg().join("saves").join("saves.json");

        if !mg_path.exists() {
            std::fs::create_dir_all(&mg_path.parent().unwrap())?;
            std::fs::File::create(&mg_path)?;
        }
        let file = std::fs::File::open(&mg_path)?;

        let reader = std::io::BufReader::new(file);

        let labels_data: Result<Labels, serde_json::Error> = serde_json::from_reader(reader);

        let save_label = Label {
            name: name.to_string(),
            uuid: get_uuid(name),
            time: chrono::Local::now().to_string(),
        };

        match labels_data {
            Ok(mut labels_data) => {
                let mut mapped = false;
                labels_data
                    .labels
                    .iter_mut()
                    .find(|f| f.name == save_label.name)
                    .map(|l| {
                        l.time = chrono::Local::now().to_string();
                        mapped = true;
                    });

                if !mapped {
                    labels_data.labels.push(save_label);
                }
                let file = std::fs::File::create(&mg_path)?;
                serde_json::to_writer(file, &labels_data)?;
            }
            Err(_) => {
                let file = std::fs::File::create(&mg_path)?;
                serde_json::to_writer(
                    file,
                    &Labels {
                        labels: vec![save_label],
                    },
                )?;
            }
        }

        Ok(())
    }

    pub fn remove_label(label: &str) -> std::io::Result<()> {
        let mg_path = crate::paths::mg().join("saves").join("saves.json");

        let label_hash = get_uuid(label);
        let mut labels = read_and_labels();
        let index = labels.labels.iter().position(|f| f.uuid == label_hash);

        if let Some(index) = index {
            labels.labels.remove(index);
            let file = fs::File::create(mg_path)?;
            serde_json::to_writer(file, &labels)?;
        } else {
            eprintln!("\x1b[31mAbort:\x1b[0m Failed to remove label: label not found");
            std::process::exit(1);
        }

        Ok(())
    }

    pub fn get_latest_label() -> std::io::Result<String> {
        let labels_data = read_and_labels();

        let mut labels = labels_data.labels.clone();
        labels.sort_by(|a, b| b.time.cmp(&a.time)); // Sort the labels by time in descending order

        if let Some(latest_label) = labels.first() {
            return Ok(latest_label.name.clone());
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No labels found",
            ));
        }
    }

    pub fn get_latest_labels() -> std::io::Result<Label> {
        let labels_data = read_and_labels();

        let mut labels = labels_data.labels.clone();
        labels.sort_by(|a, b| b.time.cmp(&a.time)); // Sort the labels by time in descending order

        if let Some(latest_label) = labels.first() {
            return Ok(latest_label.clone());
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No labels found",
            ));
        }
    }
}
