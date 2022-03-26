use inquire::MultiSelect;
use std::{
    env, fs,
    io::{self, Write},
    path::Path,
};

use crate::github::{download_file, Files};

fn write_file(file_path: &Path, content: &str) -> Result<(), io::Error> {
    let mut file = fs::File::create(&file_path)?;
    file.write_all(content.as_bytes())?;
    println!("{:?} written", &file_path.file_name().unwrap());
    Ok(())
}

pub async fn select_and_write_files(
    files_list: Files,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let options = files_list.iter().map(|v| &v.0).cloned().collect();
    let selected_options = MultiSelect::new("select files:", options).prompt()?;

    if selected_options.is_empty() {
        eprintln!("no files selected, goodbye");
        return Ok(());
    }

    let current_dir = env::current_dir()?;

    let filtered_files: Files = files_list 
        .into_iter()
        .filter(|(name, _)| selected_options.contains(name))
        .collect();

    for (file, data) in filtered_files {
        let file_path = current_dir.join(&file);
        if data.truncated {
            println!("{} is truncated, downloading...", &file);
            let downloaded_data = download_file(&data.raw_url, Some(token)).await?;
            write_file(&file_path, &downloaded_data)?;
        } else {
            write_file(&file_path, &data.content)?;
        }
    }

    Ok(())
}
