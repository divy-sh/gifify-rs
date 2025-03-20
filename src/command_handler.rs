use std::env;
use std::path::{Path, PathBuf};
use crate::gifify;

pub fn parse_args() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <input_video_path> [output_gif_path] [options]", args[0]);
        println!("Options:");
        println!("  --width <width>     Set the width of the GIF (default: 480)");
        println!("  --fps <fps>         Set the frame rate (default: 10)");
        println!("  --start <seconds>   Start time in the video");
        println!("  --duration <seconds> Duration to convert");
        println!("  --quality <1-5>     Quality level, 5 is best (default: 3)");
        return;
    }
    
    let input_path = &args[1];
    
    // Determine output path
    let output_path = if args.len() >= 3 && !args[2].starts_with("--") {
        PathBuf::from(&args[2])
    } else {
        let input = Path::new(input_path);
        let stem = input.file_stem().unwrap_or_default();
        let output_dir = input.parent().unwrap_or_else(|| Path::new(""));
        output_dir.join(format!("{}.gif", stem.to_string_lossy()))
    };
    
    // Default conversion parameters
    let mut width = 320;
    let mut fps = 10;
    let mut start_time = None;
    let mut duration = None;
    let mut quality = 3;
    
    // Parse optional parameters
    let mut i = if args.len() >= 3 && !args[2].starts_with("--") { 3 } else { 2 };
    while i < args.len() {
        match args[i].as_str() {
            "--width" => {
                if i + 1 < args.len() {
                    width = args[i + 1].parse().unwrap_or(480);
                    i += 1;
                }
            },
            "--fps" => {
                if i + 1 < args.len() {
                    fps = args[i + 1].parse().unwrap_or(10);
                    i += 1;
                }
            },
            "--start" => {
                if i + 1 < args.len() {
                    start_time = Some(args[i + 1].clone());
                    i += 1;
                }
            },
            "--duration" => {
                if i + 1 < args.len() {
                    duration = Some(args[i + 1].clone());
                    i += 1;
                }
            },
            "--quality" => {
                if i + 1 < args.len() {
                    quality = args[i + 1].parse().unwrap_or(3);
                    i += 1;
                }
            },
            _ => {
                println!("Unknown option: {}", args[i]);
            }
        }
        i += 1;
    }
    gifify::convert_to_gif(input_path, output_path, width, fps, quality, start_time, duration);
}