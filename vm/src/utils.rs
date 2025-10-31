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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};
    use tempfile::tempfile;

    #[test]
    fn test_read_i64_success() {
        let value: i64 = 1234567890123456789;
        let bytes = value.to_be_bytes();
        let mut cursor = 0;
        let result = read_i64(&bytes, &mut cursor);
        assert_eq!(result, Some(value));
        assert_eq!(cursor, 8);
    }

    #[test]
    fn test_read_i64_fail_short_slice() {
        let bytes = vec![0x01, 0x02];
        let mut cursor = 0;
        let result = read_i64(&bytes, &mut cursor);
        assert_eq!(result, None);
    }

    #[test]
    fn test_read_u32_success() {
        let value: u32 = 0xDEADBEEF;
        let bytes = value.to_be_bytes();
        let mut cursor = 0;
        let result = read_u32(&bytes, &mut cursor);
        assert_eq!(result, Some(value));
        assert_eq!(cursor, 4);
    }

    #[test]
    fn test_read_u8_success() {
        let bytes = [0xAB];
        let mut cursor = 0;
        let result = read_u8(&bytes, &mut cursor);
        assert_eq!(result, Some(0xAB));
        assert_eq!(cursor, 1);
    }

    #[test]
    fn test_read_str_with_len_success() {
        let data = b"hello";
        let mut cursor = 0;
        let result = read_str_with_len(data, &mut cursor, 5);
        assert_eq!(result, Some("hello".to_string()));
        assert_eq!(cursor, 5);
    }

    #[test]
    fn test_read_str_with_len_fail() {
        let data = b"hi";
        let mut cursor = 0;
        let result = read_str_with_len(data, &mut cursor, 5);
        assert_eq!(result, None);
    }

    #[test]
    fn test_read_u32_from_end_of_file() -> io::Result<()> {
        let mut file = tempfile()?;
        file.write_all(b"abcd")?;
        file.write_all(&0x11223344u32.to_be_bytes())?;
        file.flush()?;

        let result = read_u32_from_end_of_file(&file)?;
        assert_eq!(result, 0x11223344);
        Ok(())
    }

    #[test]
    fn test_read_n_bytes_from_end_of_file() -> io::Result<()> {
        let mut file = tempfile()?;
        file.write_all(b"abcdefg")?;
        file.flush()?;

        let result = read_n_bytes_from_end_of_file(&file, 3)?;
        assert_eq!(result, b"efg");

        Ok(())
    }

    #[test]
    fn test_read_n_bytes_from_end_of_file_too_small() -> io::Result<()> {
        let mut file = tempfile()?;
        file.write_all(b"abc")?;
        file.flush()?;

        let err = read_n_bytes_from_end_of_file(&file, 10).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::UnexpectedEof);

        Ok(())
    }
}
