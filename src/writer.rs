use inquire::MultiSelect;
use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

use crate::github::{download_file, Files};

fn write_file(file_path: PathBuf, content: &str) -> Result<(), io::Error> {
    let mut file = fs::File::create(&file_path)?;
    file.write_all(content.as_bytes())?;
    println!("{:?} written", &file_path.file_name().unwrap());
    Ok(())
}

pub async fn select_and_write_files(
    files_list: &Files,
    token: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let options = files_list.keys().cloned().collect();
    let selected = MultiSelect::new("select files:", options).prompt()?;

    let current_dir = env::current_dir()?;

    for file in selected {
        let data = files_list.get(&file).unwrap();
        let file_path = current_dir.join(&file);
        if data.truncated {
            println!("{} is truncated, downloading...", &file);
            let downloaded_data = download_file(&data.raw_url, Some(token)).await?;
            write_file(file_path, &downloaded_data)?;
        } else {
            write_file(file_path, &data.content)?;
        }
    }

    Ok(())
}
