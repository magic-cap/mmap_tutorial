use std::fs::File;
use std::{fs::OpenOptions, io::Read};

use memmap2::{Mmap, MmapOptions};
use thiserror::Error;

fn main() -> Result<(), MmapError> {
    let mut in_file = File::open("README.org")?;
    let mut contents = Vec::new();
    in_file.read_to_end(&mut contents)?;
    let mmap_in = unsafe { Mmap::map(&in_file)? };
    let temp_dir = tempfile::tempdir()?;
    let out_path = temp_dir.path().join("outfile");
    // for whatever reason, must have read even if only writing!
    let out_file = OpenOptions::new().write(true).read(true).create(true).truncate(true).open(out_path)?;
    // set the write size to the read buffer size, or crash and burn
    out_file.set_len(mmap_in.len() as u64)?;
    // println!("gonna write to {:?}",out_file);
    let mut mmap_out = unsafe { MmapOptions::new().map_mut(&out_file)? };
    mmap_out[..].copy_from_slice(&mmap_in[..]);
    mmap_out.flush()?;
    println!("mmap read into memory and mmap write out");
    Ok(())
}

#[derive(Error,Debug)]
pub enum MmapError {
    #[error("io unhappy")]
    IoError(#[source] #[from] std::io::Error),

}
