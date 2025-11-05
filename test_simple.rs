use std::fs;

fn main() {
    println!("Testing WOFF2 conversion...\n");

    // Read the test font
    let font_path = "test-font/MiSans-Demibold.ttf";
    let font_data = fs::read(font_path).expect("Failed to read font file");
    println!("✅ Font loaded: {} bytes", font_data.len());

    // Check magic number
    if font_data.len() < 4 {
        eprintln!("❌ Font file too short");
        return;
    }

    let magic = &font_data[0..4];
    println!("Magic number: {:02x} {:02x} {:02x} {:02x}", magic[0], magic[1], magic[2], magic[3]);

    // Basic validation
    let is_valid = matches!(
        magic,
        [0x00, 0x01, 0x00, 0x00]
            | [b'O', b'T', b'T', b'O']
            | [b't', b'r', b'u', b'e']
            | [b't', b'y', b'p', b'1']
    );

    if !is_valid {
        eprintln!("❌ Invalid font format");
        return;
    }
    println!("✅ Font format validated\n");

    // Read num_tables
    if font_data.len() < 6 {
        eprintln!("❌ Font data too short to read num_tables");
        return;
    }
    let num_tables = u16::from_be_bytes([font_data[4], font_data[5]]);
    println!("Number of tables: {}", num_tables);

    // Test Brotli compression
    println!("\nTesting Brotli compression...");
    let params = brotli::enc::BrotliEncoderParams {
        quality: 11,
        ..Default::default()
    };

    let mut compressed = Vec::new();
    match brotli::BrotliCompress(&mut &font_data[..], &mut compressed, &params) {
        Ok(_) => {
            println!("✅ Brotli compression successful");
            println!("   Original: {} bytes", font_data.len());
            println!("   Compressed: {} bytes", compressed.len());
            println!("   Ratio: {:.1}%", (1.0 - compressed.len() as f64 / font_data.len() as f64) * 100.0);
        }
        Err(e) => {
            eprintln!("❌ Brotli compression failed: {}", e);
            return;
        }
    }

    // Build WOFF2 header
    println!("\nBuilding WOFF2 header...");
    
    const WOFF2_HEADER_SIZE: usize = 48;
    let mut woff2_data = Vec::with_capacity(WOFF2_HEADER_SIZE + compressed.len());
    
    // Signature 'wOF2'
    woff2_data.extend_from_slice(&[b'w', b'O', b'F', b'2']);
    
    // Flavor (copy from original font)
    woff2_data.extend_from_slice(&magic);
    
    // Length (will be updated later)
    woff2_data.extend_from_slice(&[0u8; 4]);
    
    // Num tables
    woff2_data.extend_from_slice(&num_tables.to_be_bytes());
    
    // Reserved
    woff2_data.extend_from_slice(&[0u8; 2]);
    
    // Total SFNT size
    woff2_data.extend_from_slice(&(font_data.len() as u32).to_be_bytes());
    
    // Total compressed size
    woff2_data.extend_from_slice(&(compressed.len() as u32).to_be_bytes());
    
    // Major/Minor version
    woff2_data.extend_from_slice(&1u16.to_be_bytes());
    woff2_data.extend_from_slice(&0u16.to_be_bytes());
    
    // Meta offset/length/orig_length
    woff2_data.extend_from_slice(&[0u8; 12]);
    
    // Priv offset/length
    woff2_data.extend_from_slice(&[0u8; 8]);
    
    // Add compressed data
    woff2_data.extend_from_slice(&compressed);
    
    // Update length field
    let total_length = woff2_data.len() as u32;
    woff2_data[8..12].copy_from_slice(&total_length.to_be_bytes());
    
    println!("✅ WOFF2 file built: {} bytes", woff2_data.len());
    
    // Verify header
    println!("\nWOFF2 Header:");
    println!("  Signature: {}{}{}{}", 
        woff2_data[0] as char, woff2_data[1] as char, 
        woff2_data[2] as char, woff2_data[3] as char);
    println!("  Flavor: {:02x} {:02x} {:02x} {:02x}", 
        woff2_data[4], woff2_data[5], woff2_data[6], woff2_data[7]);
    let length = u32::from_be_bytes([woff2_data[8], woff2_data[9], woff2_data[10], woff2_data[11]]);
    println!("  Length: {}", length);
    let num_tables_check = u16::from_be_bytes([woff2_data[12], woff2_data[13]]);
    println!("  Num tables: {}", num_tables_check);
    
    // Save to file
    let output_path = "test-font/MiSans-Demibold.woff2";
    fs::write(output_path, &woff2_data).expect("Failed to write WOFF2 file");
    println!("\n✅ Saved to: {}", output_path);
}
