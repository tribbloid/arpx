use arpx::Runtime;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting arpx runtime to execute minimal.yaml...");

    // Load the YAML profile
    let profile_path = Path::new("examples/minimal.yaml");

    let abs = std::fs::canonicalize(profile_path)?;

    // Create runtime from the YAML profile file
    // The second argument specifies which jobs to run
    let job_names = vec!["foo".to_string()];
    let runtime = Runtime::from_profile(abs.to_str().ok_or("Invalid path")?, &job_names)?;

    println!("✅ YAML profile loaded successfully!");
    println!("⚡ Starting execution...");

    // Execute the runtime
    runtime.run()?;

    println!("🎉 Runtime execution completed successfully!");

    Ok(())
}
