use std::fs::File;
use std::io::Read;

use memmap2::Mmap;
use thiserror::Error;

fn main() -> Result<(), MmapError> {
    let mut file = File::open("README.org")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    let mmap = unsafe { Mmap::map(&file)? };
    assert_eq!(&contents[..], &mmap[..]);
    println!("Hello, world!");
    Ok(())
}

#[derive(Error,Debug)]
pub enum MmapError {
    #[error("io unhappy")]
    IoError(#[source] #[from] std::io::Error),

}
