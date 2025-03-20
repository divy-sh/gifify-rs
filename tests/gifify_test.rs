use std::path::PathBuf;
use std::fs;
use std::sync::Mutex;

// Import the function to test
use gifify_rs::gifify::convert_to_gif;

#[test]
fn test_convert_to_gif_basic() {
    let _guard = TEST_MUTEX.lock().unwrap();

    let input_path = "./assets/sample.mp4";
    let output_path = PathBuf::from("test_output.gif");
    let width = 480;
    let fps = 24;
    let quality = 2;

    convert_to_gif(input_path, output_path.clone(), width, fps, quality, None, None);

    // Check if output file is created (if ffmpeg runs correctly)
    assert!(output_path.exists(), "Output GIF file was not created");
    
    // Cleanup
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_convert_to_gif_with_options() {
    let _guard = TEST_MUTEX.lock().unwrap();

    let input_path = "./assets/sample.mp4";
    let output_path = PathBuf::from("test_output.gif");
    let width = 720;
    let fps = 30;
    let quality = 5;
    let start_time = Some("00:00:05".to_string());
    let duration = Some("10".to_string());

    convert_to_gif(input_path, output_path.clone(), width, fps, quality, start_time, duration);

    // Check if output file is created
    assert!(output_path.exists(), "Output GIF file was not created");
    
    // Cleanup
    let _ = fs::remove_file(output_path);
}

#[test]
fn test_convert_to_gif_invalid_input() {
    let _guard = TEST_MUTEX.lock().unwrap();

    let input_path = "non_existent_file.mp4";
    let output_path = PathBuf::from("test_output.gif");
    let width = 480;
    let fps = 24;
    let quality = 10;

    convert_to_gif(input_path, output_path.clone(), width, fps, quality, None, None);
    
    // The output should not be created since input is invalid
    assert!(!output_path.exists(), "Output GIF file should not be created for an invalid input");
}

static TEST_MUTEX: Mutex<()> = Mutex::new(());