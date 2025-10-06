use std::{fs::File, io::{self, Read, Seek}};

pub fn read_i64(bytes: &[u8], cursor: &mut usize) -> Option<i64> {
    if let Some(slice) = bytes
        .get(*cursor..*cursor + 8)
        .and_then(|s| s.try_into().ok())
    {
        let arr_of_bytes: [u8; 8] = slice;
        *cursor += 8;
        Some(i64::from_be_bytes(arr_of_bytes))
    } else {
        None
    }
}

pub fn read_u32(bytes: &[u8], cursor: &mut usize) -> Option<u32> {
    if let Some(slice) = bytes
        .get(*cursor..*cursor + 4)
        .and_then(|s| s.try_into().ok())
    {
        let arr_of_bytes: [u8; 4] = slice;
        *cursor += 4;
        Some(u32::from_be_bytes(arr_of_bytes))
    } else {
        None
    }
}

pub fn read_u8(bytes: &[u8], cursor: &mut usize) -> Option<u8> {
    if let Some(slice) = bytes
        .get(*cursor..*cursor + 1)
        .and_then(|s| s.try_into().ok())
    {
        let arr_of_bytes: [u8; 1] = slice;
        *cursor += 1;
        Some(u8::from_be_bytes(arr_of_bytes))
    } else {
        None
    }
}

pub fn read_str_with_len(bytes: &[u8], cursor: &mut usize, str_len: usize) -> Option<String>{
    let mut string = String::from("");
    for _ in  0..str_len {
        let char_u8 = read_u8(bytes, cursor)?;
        string.push(char_u8 as char);
    }

    Some(string)
}

pub fn read_u32_from_end_of_file(mut f: &File) -> std::io::Result<u32> {
    f.seek(std::io::SeekFrom::End(-4))?;
    let mut buf = [0u8; 4];
    f.read_exact(&mut buf).expect("Error in read byte");
    let num = u32::from_be_bytes(buf);
    Ok(num)
}

pub fn read_n_bytes_from_end_of_file(mut f: &File, n: u64) -> std::io::Result<Vec<u8>>{
    let metadata = f.metadata()?;
    let file_size = metadata.len();

    if n > file_size {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "file smaller than expected"));
    };

    f.seek(io::SeekFrom::End(-(n as i64)))?;

    let mut buf = vec![0u8; n as usize];
    f.read_exact(&mut buf).expect("Error in read byte");

    Ok(buf)
}
