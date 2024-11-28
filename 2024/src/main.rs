use glob::glob;
use std::process::Command;

fn main() {
    // Define the glob pattern to search for main.rs files
    let pattern = "./day*/**/main.rs"; // Adjust the pattern as needed

    // Iterate over the paths that match the glob pattern
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // Print the found path
                println!("Found: {}", path.display());

                // Extract the package name (e.g., "day01") from the path
                let package_name = path.components()
                    .filter_map(|component| component.as_os_str().to_str()) // Convert OsStr to &str
                    .collect::<Vec<&str>>() // Collect components into a Vec
                    .get(0) // Get the third component (index 2), which should be "day01"
                    .map_or("not found", |&v| v); // Handle case where it's not found

                // Run the cargo command
                let output = Command::new("cargo")
                    .arg("run")
                    .arg("--package")
                    .arg(package_name)
                    .arg("--bin")
                    .arg(package_name)
                    .output()
                    .expect("Failed to execute command");

                // Print any error messages from running the command
                if !output.stderr.is_empty() {
                    println!("Error output: {}", String::from_utf8_lossy(&output.stderr));
                }

                // Optionally print standard output if needed
                if !output.stdout.is_empty() {
                    println!("{}: {}", package_name, String::from_utf8_lossy(&output.stdout));
                }
            }
            Err(e) => println!("Error matching path: {}", e),
        }
    }
}
