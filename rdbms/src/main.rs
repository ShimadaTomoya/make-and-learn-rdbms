use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::path::Path;

pub struct DiskManager {
    heap_file: File, // ヒープファイルのファイルディスクリプタ
    next_page_id: u64, // 採番するページIDを決めるカウンタ
}

impl DiskManager {
    pub fn open(heap_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(heap_file_path)?;
            Self::new(heap_file_path);
    }
}

fn main() {
    println!("Hello, world!");
}
