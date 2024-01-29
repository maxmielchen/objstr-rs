use std::{fs::File, io::{Error, ErrorKind, Read as _, Seek as _, SeekFrom, Write as _}};

pub const EMPTY: [u8; 1] = [0; 1];
pub const OP_LEN: u8 = 4;

pub fn write_empty_byte(file: &mut File) {
    file.write(&EMPTY).unwrap();
}

pub fn cut(file: &mut File) {
    
    let pos = file.seek(SeekFrom::Current(0)).unwrap();

    file.set_len(pos).unwrap();
    write_empty_byte(file); 
}

pub fn truncate(file: &mut File) {
    file.set_len(0).unwrap();
    write_empty_byte(file);
    jump_stream_start(file);
}

pub fn jump_stream_start(file: &mut File) {
    file.seek(SeekFrom::Start(0)).unwrap();
}

pub fn jump_stream_end(file: &mut File) {
    file.seek(SeekFrom::End(-1)).unwrap();
}

pub fn catch_stream_read(file: &mut File, read: usize, predict: usize) -> Result<(), Error> {
    if read != predict {
        jump_stream_start(file);
        return Err(
            Error::new(
                ErrorKind::UnexpectedEof,
                "Readed data length mismatch."
            )
        );
    }
    Ok(())
}

pub fn write(file: &mut File, data: Vec<u8>) -> Result<(), Error> {

    if data.len() > u32::MAX as usize {
        return Err(
            Error::new(
                ErrorKind::InvalidInput,
                "Data length is too long."
            )
        );
    }

    let mut obj = Vec::new();
    let op = u32::to_be_bytes(data.len() as u32);

    obj.extend_from_slice(&op);
    obj.extend_from_slice(&data);
    obj.extend_from_slice(&op);

    file.write(&obj).unwrap();

    Ok(())
}

pub fn read(file: &mut File) -> Result<Vec<u8>, Error> {

    let mut len_buf_left: [u8; OP_LEN as usize] = [0; OP_LEN as usize];
    let res = file.read(&mut len_buf_left);
    catch_stream_read(
        file,
        res?,
        OP_LEN as usize
    )?;
    let len_left = u32::from_be_bytes(len_buf_left) as usize;

    let mut data = vec![0; len_left];
    let res = file.read(&mut data);
    catch_stream_read(
        file,
        res?,
        len_left
    )?;

    let mut len_buf_right: [u8; OP_LEN as usize] = [0; OP_LEN as usize];
    let res = file.read(&mut len_buf_right);
    catch_stream_read(
        file,
        res?,
        OP_LEN as usize
    )?;
    let len_right = u32::from_be_bytes(len_buf_right) as usize;
    
    if len_left != len_right {
        panic!("Data length mismatch.");
    }

    Ok(data)
}

pub fn seek_forward(file: &mut File) -> Result<(), Error> {
    let mut len_buf: [u8; OP_LEN as usize] = [0; OP_LEN as usize];

    let res = file.read(&mut len_buf);
    catch_stream_read(
        file,
        res?,
        OP_LEN as usize
    )?;
    let len = u32::from_be_bytes(len_buf) as i64;

    file.seek(SeekFrom::Current(len)).unwrap();

    file.seek(SeekFrom::Current(OP_LEN as i64)).unwrap();

    Ok(())
}

pub fn seek_backward(file: &mut File) -> Result<(), Error> {
    let mut len_buf: [u8; OP_LEN as usize] = [0; OP_LEN as usize];

    file.seek(SeekFrom::Current(-(OP_LEN as i64))).unwrap();

    let res = file.read(&mut len_buf);
    catch_stream_read(
        file,
        res?,
        OP_LEN as usize
    )?;

    let len = u32::from_be_bytes(len_buf) as i64;

    file.seek(SeekFrom::Current(-len -2*OP_LEN as i64))?;

    Ok(())
}

pub fn seek_forward_n(file: &mut File, n: u64) -> Result<(), Error> {
    for _ in 0..n {
        seek_forward(file)?;
    }
    Ok(())
}

pub fn seek_backward_n(file: &mut File, n: u64) -> Result<(), Error> {
    for _ in 0..n {
        seek_backward(file)?;
    }
    Ok(())
}

pub fn inner_len(file: &mut File) -> Result<u32, Error> {
    let mut len_buf: [u8; OP_LEN as usize] = [0; OP_LEN as usize];
    let res = file.read(&mut len_buf);
    catch_stream_read(
        file,
        res?,
        OP_LEN as usize
    )?;
    let len = u32::from_be_bytes(len_buf) as u32;

    file.seek(SeekFrom::Current(-(OP_LEN as i64)))?;

    Ok(len)
}

pub fn len_calc(inner_lens: Vec<u32>, op_sets: u8) -> i64 {
    let mut len = 0;
    for inner_len in inner_lens {
        len += inner_len as u64 + 2*OP_LEN as u64;
    }
    let len = len as i64 - OP_LEN as i64 * 2 * op_sets as i64;
    len
}
