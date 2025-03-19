# Gifify-rs

![GitHub license](https://img.shields.io/github/license/divy-sh/gifify-rs)
![GitHub stars](https://img.shields.io/github/stars/divy-sh/gifify-rs)
![GitHub issues](https://img.shields.io/github/issues/divy-sh/gifify-rs)

Gifify-rs is a Rust-based command-line tool for converting videos into high-quality GIFs efficiently.

## Features
- Fast and efficient video-to-GIF conversion
- Adjustable frame rate and quality settings
- Customizable output size
- Cross-platform support

## Installation

### Prerequisites
- Rust (Latest stable version recommended)
- FFmpeg installed on your system

### Build from Source
```sh
git clone https://github.com/divy-sh/gifify-rs.git
cd gifify-rs
cargo build --release
```

### Install via Cargo
```sh
cargo install --git https://github.com/divy-sh/gifify-rs.git
```

## Usage

Convert a video to GIF with default settings:
```sh
gifify-rs input.mp4 output.gif
```

Specify custom frame rate and resolution:
```sh
gifify-rs input.mp4 output.gif --fps 15 --width 480 --height 320
```

Adjust quality settings: [1 .. 5] [low .. high] | default = 3
```sh
gifify-rs input.mp4 output.gif --quality 5 
```

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.