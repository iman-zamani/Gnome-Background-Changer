# Gnome-Background-Changer

# Gnome-Background-Changer

A simple Rust application that automatically rotates your GNOME desktop wallpaper every minute. It scans a directory for image files (`.jpg`, `.jpeg`, `.png`) and sets them as your desktop background in sequence.

## Features
- Automatically detects the executable's directory and creates an `images` folder if it doesn't exist.
- Scans the `images` folder for compatible image files.
- Updates both the light and dark GNOME desktop wallpaper settings.
- Rotates wallpaper every 60 seconds.

## Prerequisites
- A GNOME desktop environment.
- **(For Developers)** Rust is required only if you want to build the application from source.
- Images placed in the `images` directory next to the executable.

## Installation
### Using Precompiled Releases
1. Download the latest precompiled binary for your platform from the [Releases](https://github.com/iman-zamani/Gnome-Background-Changer/releases) page.
2. Extract the binary to your desired directory.

### Building from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/iman-zamani/Gnome-Background-Changer.git
   cd Gnome-Background-Changer
   ```
2. Build the application:
   ```bash
   cargo build --release
   ```
3. Place the executable in your desired directory.

## Usage
1. Run the program:
   ```bash
   ./Gnome-Background-Changer
   ```
2. Add image files (`.jpg`, `.jpeg`, `.png`) to the `images` directory that the program creates in the executable's directory.
3. The program will rotate your desktop wallpaper every 60 seconds.

## Notes
- Ensure the `gsettings` command-line tool is available on your system.
- The program runs in an infinite loop; use `Ctrl+C` to stop it.

## Contributing
Feel free to open issues or submit pull requests to improve this project.
