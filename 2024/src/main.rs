use glob::glob;
use std::process::Command;

fn main() {
    // Define the glob pattern to search for main.rs files
    let pattern = "./src/bin/*.rs"; // Adjust the pattern as needed

    // Iterate over the paths that match the glob pattern
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {

                // Extract the package name (e.g., "day01") from the path
                let package_name = path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or("not found");

                // run cargo command to run each program as a script
                let output = Command::new("cargo")
                    .arg("run")
                    .arg("--bin")
                    .arg(package_name)
                    .output()
                    .expect("Failed to execute command");

                // Write results to stdout if they exist
                if !output.stdout.is_empty() {
                    println!("{}: {}", package_name, String::from_utf8_lossy(&output.stdout));
                }
            }
            Err(e) => println!("Error matching path: {}", e),
        }
    }
}
