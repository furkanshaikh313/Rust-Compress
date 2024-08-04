use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use zip::{ZipWriter, CompressionMethod, write::FileOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let zip_file_path = Path::new("compressed_files.zip");
    let zip_file = File::create(&zip_file_path)?;

    let mut zip = ZipWriter::new(zip_file);

    // Define the files you want to compress.
    let files_to_compress: Vec<PathBuf> = vec![
        PathBuf::from("exampleImage.png"),
        PathBuf::from(".gitignore"),
        // Add more files as needed
    ];

    // Set compression options (e.g., compression method)
    let options = FileOptions::default()
        .compression_method(CompressionMethod::DEFLATE);

    // Iterate through the files and add them to the ZIP archive.
    for file_path in &files_to_compress {
 
        let file = File::open(file_path)?;
        let file_name = file_path.file_name().unwrap().to_str().unwrap();

        // Adding the file to the ZIP archive.
        zip.start_file(file_name, options)?;

        let mut buffer = Vec::new();
        io::copy(&mut file.take(u64::MAX), &mut buffer)?;

        zip.write_all(&buffer)?;
    }

    zip.finish()?;

    println!("Files compressed successfully to {:?}", zip_file_path);

    Ok(())
}
