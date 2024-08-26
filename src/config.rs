use std::fs;

use crate::{
    logger::print_flashcard_file_summary,
    types::config_file::{ConfigFile, ConfigFlashcardFile},
    yaml_parse::parse_config_yaml,
};

pub fn does_config_file_exist() -> bool {
    let config_file_location =
        format!("{}/.flashcards/config.yaml", std::env::var("HOME").unwrap());
    fs::metadata(&config_file_location).is_ok()
}

pub fn get_matching_flashcard_file_location(flashcards_name: String) -> Option<String> {
    let config_file_location =
        format!("{}/.flashcards/config.yaml", std::env::var("HOME").unwrap());
    let file_exists = fs::metadata(&config_file_location).is_ok();

    let mut flashcard_file_location = None;

    if file_exists {
        let raw_config_yaml_result = fs::read_to_string(&config_file_location);
        let raw_config_yaml = match raw_config_yaml_result {
            Ok(raw_config_yaml) => raw_config_yaml,
            Err(e) => {
                println!("Error reading config file: {}", e);
                println!("Attempted to read file: {}", config_file_location);
                return None;
            }
        };
        let config_file: ConfigFile = parse_config_yaml(raw_config_yaml);

        for flashcard_file in config_file.flashcard_files {
            if flashcard_file.name == flashcards_name {
                flashcard_file_location = Some(flashcard_file.location);
            }
        }
    } else {
        println!("No flashcard files found");
    }

    flashcard_file_location
}

pub fn add_flashcard_file(filename: String) {
    println!("Adding flashcard file: {}", filename);

    let config_file_location =
        format!("{}/.flashcards/config.yaml", std::env::var("HOME").unwrap());
    let file_exists = does_config_file_exist();

    if file_exists {
        // read the file
        let raw_config_yaml_result = fs::read_to_string(&config_file_location);
        let raw_config_yaml = match raw_config_yaml_result {
            Ok(raw_config_yaml) => raw_config_yaml,
            Err(e) => {
                println!("Error reading config file: {}", e);
                println!("Attempted to read file: {}", config_file_location);
                return;
            }
        };
        let config_file: ConfigFile = parse_config_yaml(raw_config_yaml);

        // add the filename to the config file
        let updated_flashcard_files: Vec<ConfigFlashcardFile> = config_file
            .flashcard_files
            .iter()
            .cloned()
            .chain(vec![ConfigFlashcardFile {
                location: filename.clone(),
                name: filename.clone(),
            }])
            .collect();

        let new_config_file = ConfigFile {
            flashcard_files: updated_flashcard_files,
        };
        let new_config_file_yaml = serde_yaml::to_string(&new_config_file).unwrap();
        fs::write(&config_file_location, new_config_file_yaml).unwrap();
    } else {
        // create the file
        let new_config_file = ConfigFile {
            flashcard_files: vec![ConfigFlashcardFile {
                location: filename.clone(),
                name: filename.clone(),
            }],
        };
        let new_config_file_yaml = serde_yaml::to_string(&new_config_file).unwrap();
        fs::create_dir(format!("{}/.flashcards", std::env::var("HOME").unwrap())).unwrap();
        fs::write(&config_file_location, new_config_file_yaml).unwrap();
    }
    println!("Added flashcard file: {}", filename);
}

pub fn list_flashcard_files() {
    let config_file_location =
        format!("{}/.flashcards/config.yaml", std::env::var("HOME").unwrap());
    let file_exists = does_config_file_exist();

    if file_exists {
        let raw_config_yaml_result = fs::read_to_string(&config_file_location);
        let raw_config_yaml = match raw_config_yaml_result {
            Ok(raw_config_yaml) => raw_config_yaml,
            Err(e) => {
                println!("Error reading config file: {}", e);
                println!("Attempted to read file: {}", config_file_location);
                return;
            }
        };
        let config_file: ConfigFile = parse_config_yaml(raw_config_yaml);

        println!("Flashcard files:");
        for flashcard_file in config_file.flashcard_files {
            print_flashcard_file_summary(flashcard_file);
        }
    } else {
        println!("No flashcard files found");
    }
}

pub fn get_list_of_flashcard_files() -> Vec<ConfigFlashcardFile> {
    let config_file_location =
        format!("{}/.flashcards/config.yaml", std::env::var("HOME").unwrap());
    let file_exists = does_config_file_exist();

    if file_exists {
        let raw_config_yaml_result = fs::read_to_string(&config_file_location);
        let raw_config_yaml = match raw_config_yaml_result {
            Ok(raw_config_yaml) => raw_config_yaml,
            Err(e) => {
                println!("Error reading config file: {}", e);
                println!("Attempted to read file: {}", config_file_location);
                return vec![];
            }
        };
        let config_file: ConfigFile = parse_config_yaml(raw_config_yaml);

        println!("Flashcard files:");
        return config_file.flashcard_files;
    } else {
        return vec![];
    }
}
