use std::io::{Error, SeekFrom};

pub trait ObjStr {

    /// Seeks to the given object in the stream.
    fn seek(&mut self, pos: SeekFrom) -> Result<(), Error>;

    /// Read the next object from the stream.
    fn read(&mut self) -> Result<Vec<u8>, Error>;

    /// Returns the length of the current object (+ the following objects if objs > 1) and subtracts ops*contents from the length.
    /// 
    /// If contents::0 and objs::0, 0 is returned.
    /// 
    /// If contents::0 and objs::1, the length of the current object is returned.
    /// 
    /// If contents::1 and objs::0, error occurs.
    /// 
    /// If contents::1 and objs::1, the length of the current object-content is returned.
    /// 
    /// If contents::1 and objs::2, the length of the current object plus the length of the following object is subtracted 4*op_code(2*op_set) returned.
    /// 
    /// If contents::2 and objs::0, error occurs.
    /// 
    /// If contents::2 and objs::1, the length of the current object subtracted 4*op_code(2*op_set) is returned.
    fn len(&mut self, contents: u8, objs: u8) -> Result<u64, Error>;

    /// Overwrites the current object or objects with the given object-contents.
    /// 
    /// If objs::0, error occurs.
    /// 
    /// If objs::1 and data.len() < 1, error occurs.
    /// 
    /// If objs::1 and data.len() == 1, the current object is overwritten.
    /// 
    /// If objs::2 and data.len() == 1, the current object and the following object are overwritten.
    /// 
    /// If objs::1 and data.len() < 1, the current object will be overwritten with multiple objects.
    /// 
    /// If objs::2 and data.len() < 1, the current object and the following objects will be overwritten with multiple objects.
    fn overwrite(&mut self, data: Vec<Vec<u8>>, objs: u8) -> Result<(), Error>;

    /// Appends an object to the end of the stream.
    fn append(&mut self, data: Vec<u8>) -> Result<(), Error>;

    /// Deletes the current and all following objects.
    fn cut(&mut self) -> Result<(), Error>;
}