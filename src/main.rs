//! Biomarker Score Calculator
//!
//! This is the main entry point for the Biomarker Score Calculator application.
//! It handles command-line argument parsing and orchestrates the main workflow
//! of the program based on the user's input.

use biomarker_score_calculator::prelude::*;
use clap::{Arg, Command};
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up command-line interface
    let args = Command::new("Biomarker Score Calculator")
        .version("2.3.0")
        .about("Calculates biomarker scores based on input data and weight overrides")
        .arg(
            Arg::new("data")
                .short('d')
                .long("data")
                .value_name("PATTERN")
                .help("Glob pattern for input files (e.g. `./data/*.json`)")
                .default_value("./data/*.json"),
        )
        .arg(
            Arg::new("overrides")
                .short('o')
                .long("overrides")
                .value_name("FILE")
                .help("Optional JSON file for overriding scoring weights and other scoring conditions"),
        )
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("Run mode: 'map' to generate score map, 'overwrite' to update source files")
                .default_value("map"),
        )
        .arg(
            Arg::new("rules")
                .short('r')
                .long("rules")
                .value_name("RULES")
                .help("Optional rules file for applying custom scoring logic"))
        .get_matches();

    // Extract command-line arguments
    let glob_pattern = args.get_one::<String>("data").unwrap();
    let overrides_file_path = args.get_one::<String>("overrides");
    let weights = get_weights_overrides(overrides_file_path);
    let rules_file_path = args.get_one::<String>("rules");
    let custom_rules = parse_rules(rules_file_path);
    let mode = args.get_one::<String>("mode").unwrap();

    // Execute the appropriate function based on the run mode argument
    match mode.as_str() {
        "map" => {
            // Generate a score map and save it to a file
            let score_map = generate_score_map(glob_pattern, &weights, custom_rules).await?;
            let output_file = "biomarker_scores.json";
            let serialized_data = serde_json::to_string_pretty(&score_map)?;
            tokio::fs::write(output_file, serialized_data).await?;
            println!("Score map generated and saved to {}", output_file);
        }
        "overwrite" => {
            // Overwrite the source files with calculated scores
            overwrite_source_files(glob_pattern, &weights, custom_rules).await?;
        }
        _ => {
            // Handle invalid mode input
            println!("Invalid mode. Use 'map' or 'overwrite'.");
            process::exit(1);
        }
    }

    Ok(())
}
