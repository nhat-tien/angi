use instructions::MAGIC_NUMBER;
use shared_utils::read_byte::{read_n_bytes_from_cursor, read_n_bytes_from_end_of_file, read_str_with_len, read_u32, read_u32_from_end_of_file};
use std::{collections::HashMap, fmt::Debug, fs::File};

type EntryName = String;
type Manifest = HashMap<EntryName, Entry>;

pub struct Entry {
    pub byte: u32,
    pub offset: u32,
}

pub struct Archiver {
    blob: Vec<u8>,
    manifest: Manifest,
    manifest_count: u32,
    total_byte: u32,
    current_cursor_offset: u32
}

impl Archiver {

    pub fn new() -> Self {
        Archiver {
            blob: vec![],
            manifest: HashMap::new(),
            manifest_count: 0,
            total_byte: 0,
            current_cursor_offset: 0
        }
    }

    pub fn archive(&mut self, bytes: Vec<u8>, name: &str) {
        self.blob.extend_from_slice(&bytes);
        self.manifest.insert(
            name.to_string(),
            Entry {
            byte: bytes.len() as u32,
            offset: self.current_cursor_offset,
        });
        self.current_cursor_offset +=  bytes.len() as u32;
        self.total_byte += bytes.len() as u32;
        self.manifest_count += 1;
    }

    pub fn get_bytes(&self) -> Result<Vec<u8>, ArchiveError> {
        let mut bytes = vec![];
        let mut total_bytes = self.total_byte;

        bytes.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());

        let manifest_byte = self.get_byte_manifest();

        bytes.extend_from_slice(&self.manifest_count.to_be_bytes());
        bytes.extend_from_slice(&manifest_byte.to_be_bytes());

        total_bytes += 4; // "ANGI"
        total_bytes += 4; // "[ manifest_entry_count ]"
        total_bytes += 4; // "[ manifest_length_in_byte ]"
        total_bytes += manifest_byte;
        total_bytes += 4; // "[Total byte | u32]"

        self.inject_table_entry(&mut bytes);
        bytes.extend_from_slice(&self.blob);
        bytes.extend_from_slice(&total_bytes.to_be_bytes());

        Ok(bytes)
    }

    fn inject_table_entry(&self, bytes: &mut Vec<u8>) {
        for (entry_name, entry) in &self.manifest {
            let length_of_name: u32 = entry_name.len() as u32;

            bytes.extend_from_slice(&length_of_name.to_be_bytes());
            bytes.extend_from_slice(&entry_name.clone().into_bytes());
            bytes.extend_from_slice(&entry.offset.to_be_bytes());
            bytes.extend_from_slice(&entry.byte.to_be_bytes());
        }
    }

    fn get_byte_manifest(&self) -> u32 {
        let mut manifest_byte: u32 = 0;
        for entry_name in self.manifest.keys() {
            manifest_byte += 12 + entry_name.len() as u32;
            // 4 byte length + n byte name + 4 byte offset + 4 byte_of_blob
        };
        manifest_byte
    }

}

impl Default for Archiver {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum ArchiveError {
    Unknown { msg: String }
}


pub struct Extractor {
    blob: Vec<u8>,
    manifest: Manifest,
    blob_section_offset: u32,
}

impl Extractor {

    pub fn init_from_file(file: File) -> Result<Self, ExtractorError> {
        let blob = Extractor::get_blob_from_file(file)?;
        let (manifest, manifest_size_in_byte) = Extractor::get_manifest_from_blob(&blob)?;
        let blob_section_offset = 4*3 + manifest_size_in_byte;
        Ok(Extractor {
            blob,
            manifest,
            blob_section_offset
        })
    }

    fn get_blob_from_file(file: File) -> Result<Vec<u8>, ExtractorError> {
        let blob_size =
            read_u32_from_end_of_file(&file).map_err(|_| ExtractorError::ReadFile {
                message: "Error in get blob size".into(),
            })?;

        let bytes = read_n_bytes_from_end_of_file(&file, blob_size as u64).map_err(|_| {
            ExtractorError::ReadFile {
                message: "Error in get blob".into(),
            }
        })?;

        println!("{}", blob_size);

        Ok(bytes)
    }

    fn get_manifest_from_blob(blob: &[u8]) -> Result<(Manifest, u32), ExtractorError> {
        let mut manifest = HashMap::new();
        let mut cursor: usize = 0;

        let magic_code = read_u32(blob, &mut cursor).ok_or_else(|| {
            ExtractorError::ReadByte {
                message: "Error in get number magic code".into(),
            }
        })?;

        if magic_code != MAGIC_NUMBER {
            return Err(
                ExtractorError::UnmatchMagicNumber
            );
        };

        let manifest_size = read_u32(blob, &mut cursor).ok_or_else(|| {
            ExtractorError::ReadByte {
                message: "Error in get size of manifest".into(),
            }
        })?;

        let manifest_size_in_byte = read_u32(blob, &mut cursor).ok_or_else(|| {
            ExtractorError::ReadByte {
                message: "Error in get size in byte of manifest".into(),
            }
        })?;

        for _ in 0..manifest_size {

            let str_len = read_u32(blob, &mut cursor).ok_or_else(|| {
                ExtractorError::ReadByte {
                    message: "Error in get str len".into(),
                }
            })?;

            let string = read_str_with_len(blob, &mut cursor, str_len as usize)
                .ok_or_else(|| ExtractorError::ReadByte {
                    message: "Error in get string".into(),
                })?;

            let offset = read_u32(blob, &mut cursor).ok_or_else(|| {
                ExtractorError::ReadByte {
                    message: "Error in offset of blob".into(),
                }
            })?;

            let byte = read_u32(blob, &mut cursor).ok_or_else(|| {
                ExtractorError::ReadByte {
                    message: "Error in byte of blob".into(),
                }
            })?;

            manifest.insert(string,
                Entry {
                    offset,
                    byte
                }
            );
        }


        Ok((manifest, manifest_size_in_byte))
    }

    pub fn extract_blob(&self, name: String) -> Option<Vec<u8>> {
        let (byte, offset) = self.manifest.get(&name).map(|e| (e.byte, e.offset))?;
        let cursor = self.blob_section_offset + offset;

        read_n_bytes_from_cursor(&self.blob, cursor as usize, byte as usize)
    }

    pub fn print_debug(&self) {
        for (key, value) in &self.manifest {
           println!("key: {}\nbyte: {}\noffset: {}", key, value.byte, value.offset);
        };
    }
}


#[derive(Debug)]
pub enum ExtractorError {
    ReadFile { message: String },
    Unknown { message: String },
    ReadByte { message: String },
    UnmatchMagicNumber
}
