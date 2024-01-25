

pub trait ObjStr {

    /// Read the next object from the stream.
    fn read(&mut self) -> Vec<u8>;

    /// Overwrites the object at the current position.
    /// The object must be the same size as the original object.
    fn write(&mut self, data: Vec<u8>);

    /// Appends an object to the end of the stream.
    fn append(&mut self, data: Vec<u8>);
}