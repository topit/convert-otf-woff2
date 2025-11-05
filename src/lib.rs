use std::env;
use std::fs;
use woff::version2::compress;

#[no_mangle]
pub fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 1 {
        eprintln!("No input file specified");
        return;
    }
    
    let input_path = format!("/tmp/fonts/{}", args[0]);
    let output_dir = format!("/tmp/{}", args[0]);
    
    eprintln!("Reading font from: {}", input_path);
    
    // Read input font
    let font_data = match fs::read(&input_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            return;
        }
    };
    
    eprintln!("Input size: {} bytes", font_data.len());
    
    // Convert to WOFF2 using the woff crate
    let woff2_data = match compress(&font_data, String::new(), 11, true) {
        Some(data) => data,
        None => {
            eprintln!("Error: WOFF2 compression failed");
            return;
        }
    };
    
    eprintln!("Output size: {} bytes", woff2_data.len());
    eprintln!("Compression ratio: {:.1}%", (1.0 - woff2_data.len() as f64 / font_data.len() as f64) * 100.0);
    
    // Create output directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&output_dir) {
        eprintln!("Error creating output directory: {}", e);
        return;
    }
    
    // Write output file with .woff2 extension
    let output_path = format!("{}/font.woff2", output_dir);
    match fs::write(&output_path, &woff2_data) {
        Ok(_) => {
            eprintln!("Successfully wrote {}", output_path);
        }
        Err(e) => {
            eprintln!("Error writing output file: {}", e);
        }
    }
}

#[no_mangle]
pub fn _start() {
    main()
}
