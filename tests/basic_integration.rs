#[test]
fn test_binary_exists() {
    // Simple test to verify the binary was built
    assert!(std::path::Path::new("./target/release/tkr").exists());
}

#[test]
fn test_ls_command_basic() {
    use std::process::Command;
    
    // Test that ls command works without crashing
    let output = Command::new("./target/release/tkr")
        .arg("ls")
        .output()
        .expect("Failed to execute tkr ls command");
    
    assert!(output.status.success());
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    // Should contain some ticket listings
    assert!(output_str.contains("t-") || output_str.contains("No tickets found"));
}
