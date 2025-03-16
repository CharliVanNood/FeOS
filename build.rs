use std::fs;

fn main() {
    let version_file = "version.txt";

    let current_version = fs::read_to_string(version_file)
        .unwrap_or_else(|_| "0.1.0".to_string());

    let parts: Vec<&str> = current_version.trim().split('.').collect();
    let mut new_number = "0".to_string();

    if parts.len() > 2 {
        if let Ok(num) = parts[2].parse::<u32>() {
            new_number = (num + 1).to_string();
        }

        let new_version = format!("{}.{}.{}", parts[0], parts[1], new_number);
        fs::write(version_file, &new_version).expect("Failed to update version file");
        println!("cargo:rustc-env=VERSION={}", new_version);
    }
}
