use std::{path::PathBuf, process::Command};

pub fn convert_to_gif(
    input_path: &str,
    output_path: PathBuf,
    width: i32,
    fps: i32,
    quality: i32,
    start_time: Option<String>,
    duration: Option<String>
) {
    println!("Converting {} to {}", input_path, output_path.display());

    // Build the ffmpeg command
    let mut command = Command::new("ffmpeg");
    command.arg("-y"); // Overwrite output file if it exists
    
    // Add start time if specified
    if let Some(time) = &start_time {
        command.arg("-ss").arg(time);
    }
    
    command.arg("-i").arg(input_path);
    
    // Add duration if specified
    if let Some(time) = &duration {
        command.arg("-t").arg(time);
    }
    
    let filter_complex = format!(
        "fps={},scale={}:-1:flags=lanczos",
        fps, width
    );
    
    command.arg("-filter_complex").arg(filter_complex);
    command.arg(output_path.as_os_str());
    
    // Execute the command
    println!("Running FFmpeg with the following configuration:");
    println!("  Width: {}", width);
    println!("  FPS: {}", fps);
    println!("  Quality level: {}", quality);
    if let Some(time) = &start_time {
        println!("  Start time: {} seconds", time);
    }
    if let Some(time) = &duration {
        println!("  Duration: {} seconds", time);
    }
    
    let status = match command.status() {
        Ok(status) => status,
        Err(e) => {
            println!("Failed to execute FFmpeg: {}", e);
            println!("Make sure FFmpeg is installed and available in your PATH");
            return;
        }
    };
    
    if status.success() {
        println!("Conversion completed successfully!");
        println!("Output saved to: {}", output_path.display());
    } else {
        println!("Conversion failed with exit code: {:?}", status.code());
    }
}