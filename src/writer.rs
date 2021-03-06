use anyhow::{Context, Result};
use inquire::MultiSelect;
use std::{env, fs, io::Write, path::Path};

use crate::github::{download_file, Files, FilesVec};

fn write_file(file_path: &Path, content: &str) -> Result<()> {
    let mut file =
        fs::File::create(file_path).with_context(|| "failed to create file: {file_path}")?;
    let file_name = file_path
        .file_name()
        .with_context(|| "could not find a filename from the path")?
        .to_str()
        .with_context(|| "could not convert filename to string")?;
    file.write_all(content.as_bytes())?;
    println!("'{file_name}' written");
    Ok(())
}

pub async fn select_and_write_files(files_list: &FilesVec, token: Option<&str>) -> Result<()> {
    let options = files_list.iter().map(|v| &v.0).collect();
    let selected_options = MultiSelect::new("select files:", options).prompt()?;

    if selected_options.is_empty() {
        eprintln!("no files selected, goodbye");
        return Ok(());
    }

    let current_dir = env::current_dir().with_context(|| {
        "failed to get current directory, make sure you have correct permissions"
    })?;

    // collect vec of references after filtering
    let filtered_files: Vec<&Files> = files_list
        .iter()
        .filter(|(name, _)| selected_options.contains(&name))
        .collect();

    for (file, data) in filtered_files {
        let file_path = current_dir.join(file);
        if data.truncated {
            println!("{file} is truncated, downloading...");
            let downloaded_data = download_file(&data.raw_url, token).await?;
            write_file(&file_path, &downloaded_data)?;
        } else {
            write_file(&file_path, &data.content)?;
        }
    }

    Ok(())
}
