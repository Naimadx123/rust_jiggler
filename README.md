# Mouse Jiggler

A simple cross-platform mouse jiggler application written in Rust using the `eframe` (egui) framework and the `enigo` library.

## Description

The Mouse Jiggler is a utility designed to prevent your computer from entering idle mode or locking the screen by simulating slight mouse movements. It runs in the background and periodically moves the mouse cursor by a single pixel and back every 30 seconds.

## Cross-Platform Support

While developed on Windows, this application is built using cross-platform libraries:
- **Rust**: A systems programming language that compiles to many platforms.
- **eframe (egui)**: A cross-platform GUI framework.
- **enigo**: A cross-platform library for simulating mouse and keyboard input.

The application can be compiled for **Windows**, **macOS**, and **Linux**.

## Features

- **Simple UI**: Easy-to-use interface with a single Start/Stop toggle button.
- **Lightweight**: Minimal resource usage.
- **Automatic Stop**: Automatically stops the jiggling thread when the application is closed.

## How to Run

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.

#### Linux Dependencies
On Linux, you may need to install additional libraries for `egui` and `enigo`:
```bash
# Example for Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libxkbcommon-dev libdbus-1-dev libatk1.0-dev libgtk-3-dev libxdo-dev
```

### Running the application

1. Clone the repository or download the source code.
2. Open a terminal in the project directory.
3. Run the following command:
   ```bash
   cargo run --release
   ```

## Usage

1. Click the **START** button to begin jiggling.
2. Click the **STOP** button to pause.
3. Closing the window will automatically stop the jiggler and exit the application.

## License

This project is licensed under the terms of the MIT license.
