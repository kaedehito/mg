use zstd::Decoder;
use std::{fs::File, path::Path};
use tar::Archive;



pub fn unpack<P: AsRef<Path>>(from: P, out: P) -> std::io::Result<()> {
    let from_path = from.as_ref();
    let to_path = out.as_ref();


    // Open the compressed file
    let file = File::open(&from_path)?;
    let decoder = Decoder::new(file)?;
    let mut archive = Archive::new(decoder);

    // Extract the contents of the archive
    archive.unpack(&to_path)?;

    Ok(())
}
