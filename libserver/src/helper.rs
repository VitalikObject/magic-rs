use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io::Write;

pub fn compress_in_zlib_format(input: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::new(1));
    encoder.write_all(input).expect("Failed to write to encoder");
    let compressed = encoder.finish().expect("Failed to finish compression");
    
    let uncompressed_length = input.len() as u32;
    let mut output = Vec::with_capacity(compressed.len() + 4);
    
    output.extend_from_slice(&uncompressed_length.to_le_bytes());
    output.extend_from_slice(&compressed);
    
    output
}