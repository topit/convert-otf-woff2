#!/usr/bin/env node

import { readFile, writeFile } from 'fs/promises';
import init, { convert_otf_to_woff2, validate_font, get_font_format } from './pkg/convert_otf_woff2.js';

async function testConversion() {
    console.log('Loading WASM module...');
    await init();
    console.log('✅ WASM module loaded\n');

    const fontPath = './test-font/MiSans-Demibold.ttf';
    console.log(`Reading font: ${fontPath}`);
    const fontData = await readFile(fontPath);
    console.log(`✅ Font loaded: ${fontData.length} bytes\n`);

    // Validate
    console.log('Validating font...');
    const isValid = validate_font(fontData);
    console.log(`Valid: ${isValid}`);
    
    const format = get_font_format(fontData);
    console.log(`Format: ${format}\n`);

    if (!isValid) {
        console.error('❌ Font validation failed');
        process.exit(1);
    }

    // Convert
    console.log('Converting to WOFF2...');
    try {
        const woff2Data = convert_otf_to_woff2(fontData);
        console.log(`✅ Conversion successful: ${woff2Data.length} bytes`);
        console.log(`Compression ratio: ${((1 - woff2Data.length / fontData.length) * 100).toFixed(1)}%\n`);

        // Check WOFF2 magic number
        const magic = Array.from(woff2Data.slice(0, 4))
            .map(b => b.toString(16).padStart(2, '0'))
            .join(' ');
        console.log(`WOFF2 magic: ${magic}`);
        console.log(`Expected: 77 4f 46 32 (wOF2)`);

        if (woff2Data[0] === 0x77 && woff2Data[1] === 0x4f && 
            woff2Data[2] === 0x46 && woff2Data[3] === 0x32) {
            console.log('✅ WOFF2 magic number is correct\n');
        } else {
            console.error('❌ WOFF2 magic number is incorrect\n');
        }

        // Show header info
        console.log('WOFF2 Header analysis:');
        const view = new DataView(woff2Data.buffer, woff2Data.byteOffset, Math.min(48, woff2Data.length));
        console.log(`  Signature: ${Array.from(woff2Data.slice(0, 4)).map(b => String.fromCharCode(b)).join('')}`);
        console.log(`  Flavor: ${Array.from(woff2Data.slice(4, 8)).map(b => '0x' + b.toString(16).padStart(2, '0')).join(' ')}`);
        console.log(`  Length: ${view.getUint32(8)}`);
        console.log(`  Num tables: ${view.getUint16(12)}`);
        console.log(`  Total SFNT size: ${view.getUint32(16)}`);
        console.log(`  Total compressed size: ${view.getUint32(20)}`);

        // Save the converted file
        const outputPath = './test-font/MiSans-Demibold.woff2';
        await writeFile(outputPath, woff2Data);
        console.log(`\n✅ Saved to: ${outputPath}`);

        // Try to validate with external tool if available
        console.log('\nTrying to validate WOFF2 file...');
        const { exec } = await import('child_process');
        exec('which woff2_decompress', (error, stdout) => {
            if (!error && stdout.trim()) {
                exec(`woff2_decompress ${outputPath}`, (err, out, stderr) => {
                    if (err) {
                        console.error('❌ woff2_decompress failed:', stderr);
                    } else {
                        console.log('✅ woff2_decompress succeeded:', out);
                    }
                });
            } else {
                console.log('⚠️  woff2_decompress not available for validation');
            }
        });

    } catch (error) {
        console.error('❌ Conversion failed:', error);
        console.error(error.stack);
        process.exit(1);
    }
}

testConversion().catch(console.error);
