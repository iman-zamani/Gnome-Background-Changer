use std::{
    env,
    fs::{self},
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::Duration,
};

fn main() {
    loop {
        // executable path
        let exe_path = env::current_exe().expect("Failed to get the executable path");
        let exe_dir = exe_path.parent().expect("Failed to get the executable directory");

        println!("Executable is located at: {}", exe_dir.display());

        // 'images' directory 
        let images_dir = exe_dir.join("images");
        if !images_dir.exists() {
            println!("No 'images' directory found, creating one.");
            fs::create_dir(&images_dir).expect("Failed to create 'images' directory");
            return; // exit the program
        }

        // List images 
        let image_files = list_image_files(&images_dir);
        if image_files.is_empty() {
            println!("No image files found in 'images' directory.");
        }
        
        for image_file in &image_files {
            let file_uri = format!("file://{}", image_file.display());

            // run the gsettings command to get the current background
            let output = Command::new("gsettings")
                .arg("get")
                .arg("org.gnome.desktop.background")
                .arg("picture-uri")
                .output()
                .expect("Failed to execute gsettings get");

            // Update the background with the new image for dark background 
            Command::new("gsettings")
                .arg("set")
                .arg("org.gnome.desktop.background")
                .arg("picture-uri-dark")
                .arg(file_uri.clone()) 
                .status()
                .expect("Failed to execute gsettings set");
             // Update the background with the new image for default background 
            Command::new("gsettings")
                .arg("set")
                .arg("org.gnome.desktop.background")
                .arg("picture-uri")
                .arg(file_uri.clone()) 
                .status()
                .expect("Failed to execute gsettings set");
            println!("Background updated to: {}", file_uri);

            // sleep for 60 seconds
            thread::sleep(Duration::from_secs(60));
        }
    }
}

fn list_image_files(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.path().extension().map_or(false, |ext| {
                ext.eq("jpg") || ext.eq("jpeg") || ext.eq("png")
            })
        })
        .map(|entry| entry.path())
        .collect()
}
