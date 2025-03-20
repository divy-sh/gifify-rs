use std::env;
use std::sync::Mutex;
use gifify_rs::command_handler::parse_args;

#[test]
fn test_parse_args_with_defaults() {
    let _guard = TEST_MUTEX.lock().unwrap(); // Ensure tests don't interfere with each other
    unsafe {
        env::set_var("CARGO_BIN_NAME", "gif_tool");
        env::set_var("CARGO_PKG_NAME", "gif_tool");
        let args: Vec<String> = vec!["gif_tool", "input.mp4"].iter().map(|s| s.to_string()).collect();
        env::set_var("ARGS", args.join(" "));
    }
    
    parse_args();
}

#[test]
fn test_parse_args_with_options() {
    let _guard = TEST_MUTEX.lock().unwrap();
    unsafe {
        env::set_var("CARGO_BIN_NAME", "gif_tool");
        env::set_var("CARGO_PKG_NAME", "gif_tool");
        let args: Vec<String> = vec![
            "gif_tool", "input.mp4", "output.gif", "--width", "500", "--fps", "15", "--quality", "4", "--start", "2", "--duration", "10"
        ].iter().map(|s| s.to_string()).collect();
        env::set_var("ARGS", args.join(" "));
    }
    parse_args();
}

static TEST_MUTEX: Mutex<()> = Mutex::new(());