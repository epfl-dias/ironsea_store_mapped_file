use std::fs::File;
use std::io;

use memmap::Mmap;

use ironsea_store::Load;

pub fn load<T: Load>(from: String) -> io::Result<T> {
    let file_in = File::open(from)?;
    let mmap = unsafe { Mmap::map(&file_in) }?;

    T::load_slice(&mmap)
}

//FIXME: Implement mmapped write. Requires to pre-allocate the memory area / file.