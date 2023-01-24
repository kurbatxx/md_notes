use serde_derive::Deserialize;
use std::{env, fs, io};

use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Clone, Debug)]
struct Config {
    github_io_url: String,
}
#[derive(Deserialize, Clone, Debug)]
struct BookConfig {
    build: Build,
}
#[derive(Deserialize, Clone, Debug)]
struct Build {
    #[serde(rename(deserialize = "build-dir"))]
    build_dir: String,
}

fn main() {
    let config_file = fs::read_to_string("config.toml").expect("Нет config файла");
    let config: Config = toml::from_str(&config_file).expect("Неправильно заполнен config.toml");

    let book_config_file = fs::read_to_string("book.toml").expect("Нет config файла");
    let book_config: BookConfig =
        toml::from_str(&book_config_file).expect("Неправильно заполнен config.toml");

    let url = config.github_io_url;
    let splited_address: Vec<&str> = url.split("/").collect();
    let repo_name = *splited_address.last().expect("Неправильный адресс");
    let repo_dir = format!("./{}", &repo_name);

    let repo_exist = Path::new(&repo_dir).is_dir();

    if !repo_exist {
        let _ = Command::new("git").args(["clone", &url]).output();
    }

    //let _ = Command::new("cd").args([repo_name]).output();
    let _ = env::set_current_dir(&repo_dir);

    let _ = Command::new("git").args(["reset", "--hard"]).output();
    let _ = Command::new("git").args(["pull"]).output();

    let book_folder_name = book_config.build.build_dir;

    let dir = format!("./{}", &book_folder_name);

    let dir_exist = Path::new(&dir).is_dir();
    if dir_exist {
        std::fs::remove_dir_all(&dir).unwrap()
    }

    let _ = copy_dir_all(format!("./../{}", &book_folder_name), dir);

    let _ = Command::new("git").args(["add", "."]).output();
    let _ = Command::new("git")
        .args(["commit", "-m", "\"update\""])
        .output();

    let _ = Command::new("git").args(["push"]).output();
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
