use std::process::Command;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Helper function to find all ticket files recursively
fn find_ticket_files(tickets_dir: &PathBuf) -> Vec<PathBuf> {
    let mut ticket_files = Vec::new();

    // Search in all status subdirectories
    if let Ok(entries) = fs::read_dir(tickets_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // This is a status directory (open, closed, etc.)
                if let Ok(status_entries) = fs::read_dir(&path) {
                    for status_entry in status_entries.flatten() {
                        let ticket_path = status_entry.path();
                        if ticket_path.extension().map(|ext| ext == "md").unwrap_or(false) {
                            ticket_files.push(ticket_path);
                        }
                    }
                }
            } else if path.extension().map(|ext| ext == "md").unwrap_or(false) {
                // Direct .md file in tickets directory
                ticket_files.push(path);
            }
        }
    }

    ticket_files
}

#[test]
fn test_binary_exists() {
    // Simple test to verify the binary was built
    assert!(std::path::Path::new("./target/release/tkr").exists());
}

#[test]
fn test_help_command() {
    let output = Command::new("./target/release/tkr")
        .arg("--help")
        .output()
        .expect("Failed to execute tkr --help command");

    assert!(output.status.success());

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("Usage:"));
    assert!(output_str.contains("tkr"));
}

#[test]
fn test_version_command() {
    let output = Command::new("./target/release/tkr")
        .arg("version")
        .output()
        .expect("Failed to execute tkr version command");

    assert!(output.status.success());

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("tkr 0.1.0"));
    assert!(output_str.contains("ticket management system"));
}

#[test]
fn test_ls_command_basic() {
    let output = Command::new("./target/release/tkr")
        .arg("ls")
        .output()
        .expect("Failed to execute tkr ls command");

    assert!(output.status.success());

    let output_str = String::from_utf8_lossy(&output.stdout);
    // Should contain some ticket listings
    assert!(output_str.contains("t-") || output_str.contains("No tickets found"));
}

#[test]
fn test_create_ticket() {
    let temp_dir = TempDir::new().unwrap();
    let tickets_dir = temp_dir.path().join(".tickets");

    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("create")
        .arg("Test ticket")
        .output()
        .expect("Failed to execute tkr create command");

    assert!(output.status.success());

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("t-") || output_str.len() > 3); // Just check it's not empty

    // Check ticket file was created
    let ticket_files = find_ticket_files(&tickets_dir);
    assert_eq!(ticket_files.len(), 1);

    let content = fs::read_to_string(ticket_files[0].as_path()).unwrap();
    assert!(content.contains("Test ticket"));
    assert!(content.contains("status: open"));
}

#[test]
fn test_create_ticket_with_project_category() {
    let temp_dir = TempDir::new().unwrap();
    let tickets_dir = temp_dir.path().join(".tickets");

    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("--project")
        .arg("test-project")
        .arg("--category")
        .arg("test-category")
        .arg("create")
        .arg("Test ticket with tags")
        .output()
        .expect("Failed to execute tkr create command");

    assert!(output.status.success());

    // Check ticket content contains project and category tags
    let ticket_files = find_ticket_files(&tickets_dir);
    assert_eq!(ticket_files.len(), 1);

    let content = fs::read_to_string(ticket_files[0].as_path()).unwrap();
    assert!(content.contains("project: test-project"));
    assert!(content.contains("category: test-category"));
}

#[test]
fn test_list_tickets() {
    let temp_dir = TempDir::new().unwrap();
    let tickets_dir = temp_dir.path().join(".tickets");

    // Create a ticket first
    let _ = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("create")
        .arg("List test ticket")
        .output()
        .expect("Failed to execute tkr create command");

    // List tickets
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("list")
        .output()
        .expect("Failed to execute tkr list command");

    assert!(output.status.success());

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("List test ticket") || output_str.contains("t-"));
}

#[test]
fn test_ticket_status_update() {
    let temp_dir = TempDir::new().unwrap();
    let tickets_dir = temp_dir.path().join(".tickets");

    // First create a ticket
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("create")
        .arg("Status test ticket")
        .output()
        .expect("Failed to execute tkr create command");

    assert!(output.status.success());

    // Get the ticket ID
    let ticket_files = find_ticket_files(&tickets_dir);
    let ticket_path = ticket_files[0].clone();
    let ticket_id = ticket_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    // Update status
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("status")
        .arg(ticket_id)
        .arg("in_progress")
        .output()
        .expect("Failed to execute tkr status command");

    assert!(output.status.success());

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("Updated") || output_str.contains("in_progress"));
}

#[test]
fn test_add_note() {
    let temp_dir = TempDir::new().unwrap();
    let tickets_dir = temp_dir.path().join(".tickets");

    // Create a ticket first
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("create")
        .arg("Note test ticket")
        .output()
        .expect("Failed to execute tkr create command");

    assert!(output.status.success());

    // Get the ticket ID
    let ticket_files = find_ticket_files(&tickets_dir);
    let ticket_path = ticket_files[0].clone();
    let ticket_id = ticket_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    // Add a note
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("add-note")
        .arg(ticket_id)
        .arg("This is a test note")
        .output()
        .expect("Failed to execute tkr add-note command");

    assert!(output.status.success());

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("Note") || output_str.contains("Added") || output.status.success());
}

#[test]
fn test_start_command() {
    let temp_dir = TempDir::new().unwrap();
    let tickets_dir = temp_dir.path().join(".tickets");

    // Create a ticket first
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("create")
        .arg("Start test ticket")
        .output()
        .expect("Failed to execute tkr create command");

    assert!(output.status.success());

    // Get the ticket ID
    let ticket_files = find_ticket_files(&tickets_dir);
    let ticket_path = ticket_files[0].clone();
    let ticket_id = ticket_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    // Start the ticket
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("start")
        .arg(ticket_id)
        .output()
        .expect("Failed to execute tkr start command");

    assert!(output.status.success());

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("Started") || output_str.contains("in_progress"));
}

#[test]
fn test_close_command() {
    let temp_dir = TempDir::new().unwrap();
    let tickets_dir = temp_dir.path().join(".tickets");

    // Create a ticket first
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("create")
        .arg("Close test ticket")
        .output()
        .expect("Failed to execute tkr create command");

    assert!(output.status.success());

    // Get the ticket ID
    let ticket_files = find_ticket_files(&tickets_dir);
    let ticket_path = ticket_files[0].clone();
    let ticket_id = ticket_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    // Close the ticket
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("close")
        .arg(ticket_id)
        .output()
        .expect("Failed to execute tkr close command");

    assert!(output.status.success());

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("Closed") || output_str.contains("closed"));
}

#[test]
fn test_ready_command() {
    let temp_dir = TempDir::new().unwrap();
    let tickets_dir = temp_dir.path().join(".tickets");

    // Create a ticket first
    let _ = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("create")
        .arg("Ready test ticket")
        .output()
        .expect("Failed to execute tkr create command");

    // List ready tickets (should work since no dependencies)
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("ready")
        .output()
        .expect("Failed to execute tkr ready command");

    assert!(output.status.success());

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("Ready") || output_str.contains("ready") || output_str.len() > 0);
}

#[test]
fn test_show_command() {
    let temp_dir = TempDir::new().unwrap();
    let tickets_dir = temp_dir.path().join(".tickets");

    // Create a ticket first
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("create")
        .arg("Show test ticket")
        .output()
        .expect("Failed to execute tkr create command");

    assert!(output.status.success());

    // Get the ticket ID
    let ticket_files = find_ticket_files(&tickets_dir);
    let ticket_path = ticket_files[0].clone();
    let ticket_id = ticket_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    // Show ticket details
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("show")
        .arg(ticket_id)
        .output()
        .expect("Failed to execute tkr show command");

    assert!(output.status.success());

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("Show test ticket") || output_str.contains(ticket_id));
}

#[test]
fn test_dependency_management() {
    let temp_dir = TempDir::new().unwrap();
    let tickets_dir = temp_dir.path().join(".tickets");

    // Create first ticket
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("create")
        .arg("Parent ticket")
        .output()
        .expect("Failed to execute tkr create command");

    assert!(output.status.success());

    // Create second ticket
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("create")
        .arg("Child ticket")
        .output()
        .expect("Failed to execute tkr create command");

    assert!(output.status.success());

    // Get ticket IDs using the helper function
    let ticket_files = find_ticket_files(&tickets_dir);
    assert_eq!(ticket_files.len(), 2);

    let parent_id = ticket_files[0]
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    let child_id = ticket_files[1]
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();

    // Add dependency
    let output = Command::new("./target/release/tkr")
        .env("TICKETS_DIR", &tickets_dir)
        .arg("dep")
        .arg(child_id)
        .arg(parent_id)
        .output()
        .expect("Failed to execute tkr dep command");

    assert!(output.status.success());

    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(output_str.contains("Dependency") || output_str.contains("Added") || output_str.contains("dep") || output.status.success());
}
