use std::{fs::{create_dir, remove_dir_all, remove_file, File}, io::{self, BufReader, BufWriter}, path::Path};
/// Trait for file operations
pub trait Pathjects {
    /// Open a file
    fn open(&self) -> io::Result<File>;
    /// Create a file
    fn create(&self) -> io::Result<File>;
    /// Open a file with a buffer reader
    fn bufread(&self) -> io::Result<BufReader<File>>;
    /// Create a file with a buffer writer
    fn bufwrite(&self) -> io::Result<BufWriter<File>>;
    /// Delete a file or directory
    fn delete(&self) -> io::Result<()>;
    /// Create a directory
    fn mkdir(&self) -> io::Result<()>;
}

impl Pathjects for Path {
    fn open(&self) -> io::Result<File> {
        File::open(self)
    }
    fn create(&self) -> io::Result<File> {
        File::create(self)
    }
    fn bufread(&self) -> io::Result<BufReader<File>> {
        Ok(BufReader::new(self.open()?))
    }
    fn bufwrite(&self) -> io::Result<BufWriter<File>> {
        Ok(BufWriter::new(self.create()?))
    }
    fn delete(&self) -> io::Result<()> {
        if self.is_file() {
            remove_file(self)
        } else if self.is_dir() {
            remove_dir_all(self)
        } else {Ok(())}
    }
    fn mkdir(&self) -> io::Result<()> {
        create_dir(self)
    }

}




#[cfg(test)]
mod test;