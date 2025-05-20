// This is a modified version of scripts/srs_downloader/src/main.rs the file from https://github.com/madztheo/noir-react-native-starter

use std::path::{Path, PathBuf};
use std::fs;

use clap::Parser;
use noir::barretenberg::{
    srs::{get_srs, localsrs::LocalSrs, netsrs::NetSrs, Srs},
    utils::get_subgroup_size,
};
use serde_json::Value;

const DEFAULT_SRS_DIR: &str = "./srs_cache";

#[derive(Parser, Debug, PartialEq)]
struct Args {
    #[clap(short, long, help = "Path to the circuit JSON manifest file.")]
    circuit_path: Option<String>,

    #[clap(short, long, help = "Specific output path to save the SRS file. If not provided, saves to a default directory.")]
    output_path: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let local_srs: LocalSrs;
    let mut circuit_name = "default_circuit".to_string();

    match &args.circuit_path {
        Some(path_str) => {
            let circuit_file_path = Path::new(path_str);
            println!("Reading circuit from: {}", circuit_file_path.display());

            let manifest = fs::read(circuit_file_path).map_err(|e| format!("Failed to read circuit file {}: {}", circuit_file_path.display(), e))?;
            let manifest_value: Value =
                serde_json::from_slice(&manifest).map_err(|e| format!("Failed to decode JSON from {}: {}", circuit_file_path.display(), e))?;
            let bytecode = manifest_value["bytecode"]
                .as_str()
                .ok_or_else(|| format!("Failed to get bytecode from {}", circuit_file_path.display()))?;

            if let Some(name) = circuit_file_path.file_stem().and_then(|s| s.to_str()) {
                circuit_name = name.to_string();
            }

            println!("Circuit '{}' decoded. Downloading SRS...", circuit_name);
            let subgroup_size = get_subgroup_size(bytecode, false);
            let srs: Srs = get_srs(subgroup_size, None);
            local_srs = LocalSrs(srs);
            println!("SRS for '{}' downloaded.", circuit_name);
        }
        None => {
            println!("No circuit path provided, using default circuit size (2^18).");
            println!("Downloading SRS...");
            // Default to around 256K constraints, which should be enough
            // for most circuits that can work on a mobile device
            // This translates to a subgroup size of 262144 (the next power of 2 above 256k, i.e. 2^18)
            let srs: Srs = NetSrs::new(262144 + 1).to_srs();
            local_srs = LocalSrs(srs);
            circuit_name = "default_18".to_string();
            println!("SRS for default size downloaded.");
        }
    }

    let save_path_buf: PathBuf = match &args.output_path {
        Some(path_str) => PathBuf::from(path_str),
        None => {
            let mut path = PathBuf::from(DEFAULT_SRS_DIR);
            path.push(format!("{}.srs", circuit_name));
            path
        }
    };
    
    if let Some(parent) = save_path_buf.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create parent directory {}: {}", parent.display(), e))?;
        }
    }

    println!("Saving SRS to {}...", save_path_buf.display());
    local_srs.save(Some(save_path_buf.to_str().ok_or("Invalid save path")?));
    println!("SRS saved to {}", save_path_buf.display());

    Ok(())
}
