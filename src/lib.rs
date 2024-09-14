use std::{fs::{create_dir, remove_dir_all, remove_file, File}, io::{self, BufReader, BufWriter}, path::{Path, PathBuf}};

// helper traits for paths:
trait Paths{}
impl Paths for Path{}
impl Paths for PathBuf{}

pub trait Pathjects {
    fn open(&self) -> io::Result<File>;
    fn create(&self) -> io::Result<File>;
    fn bufread(&self) -> io::Result<BufReader<File>>;
    fn bufwrite(&self) -> io::Result<BufWriter<File>>;
    fn delete(&self) -> io::Result<()>;
    fn mkdir(&self) -> io::Result<()>;
    
}

impl<T: Paths> Pathjects for T {
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