use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // 二次确认
    println!("Enter 'y' to continue:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim() != "y" {
        return Ok(());
    }

    let current_exe = env::current_exe()?.file_name().unwrap().to_os_string();

    let current_dir = env::current_dir()?;

    let mut hash_map = HashMap::new();

    let entries = fs::read_dir(&current_dir)?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .filter(|e| e.file_name() != current_exe && e.file_name() != "sha256.txt");

    for entry in entries {
        let path = entry.path();
        let file_name = match path.file_name() {
            Some(name) => name.to_os_string(),
            None => continue,
        };
        let extension = path.extension().map(|ext| ext.to_os_string());

        let mut file = File::open(&path)?;
        let mut hasher = Sha256::new();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        hasher.update(&buffer);
        let result = hasher.finalize();
        let hash_str = format!("{:x}", result);

        // 存储文件名和哈希值
        hash_map.insert(file_name.to_string_lossy().to_string(), hash_str.clone());

        let new_file_name = if let Some(ext) = extension {
            format!("{}.{}", hash_str, ext.to_string_lossy())
        } else {
            hash_str
        };

        let new_path = current_dir.join(&new_file_name);
        fs::rename(&path, &new_path)?;
        println!("Renamed {:?} to {:?}", path, new_path);
    }

    // 所有文件处理完毕后再写入sha256.txt
    let mut sha256_file = File::create(current_dir.join("sha256.txt"))?;
    for (file_name, hash) in hash_map {
        writeln!(sha256_file, "{}: {}", file_name, hash)?;
    }

    Ok(())
}
