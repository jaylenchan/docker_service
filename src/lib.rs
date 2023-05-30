pub mod docker_compose;
pub mod docker_file;

use anyhow::Result;
use fs_extra::dir::create_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_to_dir(dir: &str, filename: &str, content: String) -> Result<()> {
    let path = Path::new(dir);

    if !path.exists() {
        create_all(path, true)?;
    }

    let create_file_result = File::create(&path.join(filename));

    match create_file_result {
        Ok(mut file) => {
            if let Err(_) = write!(&mut file, "{}", content) {
                panic!("Failed to write file: {}", &path.to_string_lossy());
            }
        }
        Err(_) => {
            panic!("Failed to create file: {}", &path.to_string_lossy());
        }
    }

    Ok(())
}
