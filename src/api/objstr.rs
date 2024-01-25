use std::io::{Error, SeekFrom};

pub trait ObjStr {

    /// Seeks to the given object in the stream.
    fn obj_seek(&mut self, pos: SeekFrom) -> Result<(), Error>;

    /// Read the next object from the stream.
    fn obj_read(&mut self) -> Result<Vec<u8>, Error>;

    /// Overwrites the object at the current position.
    /// The object must be the same size as the original object.
    fn obj_write(&mut self, data: Vec<u8>) -> Result<(), Error>;

    /// Appends an object to the end of the stream.
    fn obj_append(&mut self, data: Vec<u8>) -> Result<(), Error>;
}