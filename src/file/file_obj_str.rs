use std::{fs::{File, OpenOptions}, io::{Error, SeekFrom}, path::Path};

use crate::api::ObjStr;

use super::helpers::{cut, inner_len, jump_stream_end, jump_stream_start, len_calc, read, seek_backward_n, seek_forward, seek_forward_n, truncate, write, write_empty_byte};

pub struct FileObjStr {
    file: File
}

impl FileObjStr {
    pub fn new(path: &Path) -> Result<FileObjStr, Error> {
        let mut file: File;

        if path.exists() {
            file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)?;
        } else {
            file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path)?;
            truncate(&mut file);
        }     

        Ok(
            FileObjStr {
                file
            }
        )
    }
}

impl ObjStr for FileObjStr {

    fn seek(&mut self, pos: SeekFrom) -> Result<(), Error> {
        
        match pos {
            SeekFrom::Start(pos) => {
                jump_stream_start(&mut self.file);
                seek_backward_n(&mut self.file, pos)?;
            },
            SeekFrom::End(pos) => {
                jump_stream_end(&mut self.file);
                seek_backward_n(&mut self.file, pos.abs() as u64)?;
            },
            SeekFrom::Current(pos) => {
                let pos_abs = pos.abs() as u64;
                if pos < 0 {
                    seek_backward_n(&mut self.file, pos_abs)?;
                } else if pos > 0 {
                    seek_forward_n(&mut self.file, pos_abs)?;
                }
            }
        }

        Ok(())

    }

    fn read(&mut self) -> Result<Vec<u8>, Error> {
        Ok(read(&mut self.file)?)
    }

    fn len(&mut self, contents: u8, objs: u8) -> Result<u64, Error> {

        let mut inner_lens = Vec::new();

        for _ in 0..objs {
            inner_lens.push(
                inner_len(&mut self.file)?
            );
            seek_forward(&mut self.file)?;
        }

        seek_backward_n(&mut self.file, objs as u64)?;

        Ok(
            len_calc(inner_lens, contents)
        )
    }

    fn overwrite(&mut self, data: Vec<Vec<u8>>, objs: u8) -> Result<(), Error> {
        todo!()
    }

    fn append(&mut self, data: Vec<u8>) -> Result<(), Error> {
        jump_stream_end(&mut self.file);
        write(&mut self.file, data)?;
        write_empty_byte(&mut self.file);
        Ok(())
    }

    fn cut(&mut self) -> Result<(), Error> {
        cut(&mut self.file);
        Ok(())
    }
}