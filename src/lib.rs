use std::{fmt::Display, fs::{create_dir, remove_dir_all, remove_file, File}, io::{self, BufReader, BufWriter, Read}, path::{Path, PathBuf}};

/// A single grep match: which line it was found on, the matched text, and its column range within that line
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    /// 1-based line number
    pub line_number: usize,
    /// The full text of the line the match was found on
    pub text: String,
    /// The matched substring
    pub matched: String,
    /// Start column (char index, 0-based) of the match within the line
    pub start: usize,
    /// End column (char index, 0-based, exclusive) of the match within the line
    pub end: usize,
}

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
    /// Search a text file for a substring, returning the byte index of the first match
    fn search(&self, needle: &str) -> io::Result<Option<usize>> {
        let mut content = String::new();
        self.open()?.read_to_string(&mut content)?;
        Ok(content.find(needle))
    }
    /// Search a file's raw bytes for a byte sequence, returning the byte index of the first match
    fn search_bin(&self, needle: &[u8]) -> io::Result<Option<usize>> {
        let mut content = Vec::new();
        self.open()?.read_to_end(&mut content)?;
        if needle.is_empty() {
            return Ok(Some(0));
        }
        Ok(content.windows(needle.len()).position(|w| w == needle))
    }
    /// Search a text file line by line for a substring, returning one `Line` per match found
    fn grep(&self, needle: &str) -> io::Result<Vec<Line>> {
        let mut content = String::new();
        self.open()?.read_to_string(&mut content)?;
        let mut matches = Vec::new();
        for (line_number, line) in content.lines().enumerate() {
            for (byte_start, matched) in line.match_indices(needle) {
                let start = line[..byte_start].chars().count();
                let end = start + matched.chars().count();
                matches.push(Line {
                    line_number: line_number + 1,
                    text: line.to_string(),
                    matched: matched.to_string(),
                    start,
                    end,
                });
            }
        }
        Ok(matches)
    }
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

impl Pathjects for PathBuf {
    fn open(&self) -> io::Result<File> {
        self.as_path().open()
    }
    fn create(&self) -> io::Result<File> {
        self.as_path().create()
    }
    fn bufread(&self) -> io::Result<BufReader<File>> {
        self.as_path().bufread()
    }
    fn bufwrite(&self) -> io::Result<BufWriter<File>> {
        self.as_path().bufwrite()
    }
    fn delete(&self) -> io::Result<()> {
        self.as_path().delete()
    }
    fn mkdir(&self) -> io::Result<()> {
        self.as_path().mkdir()
    }
}

/// Trait for string manipulation
pub trait Stringjects {
    /// True if the string is empty or contains only whitespace
    fn is_blank(&self) -> bool;
    /// Truncate to at most `max_len` chars, appending "..." if truncated
    fn truncate_ellipsis(&self, max_len: usize) -> String;
}

impl Stringjects for str {
    fn is_blank(&self) -> bool {
        self.trim().is_empty()
    }
    fn truncate_ellipsis(&self, max_len: usize) -> String {
        let char_count = self.chars().count();
        if char_count <= max_len {
            self.to_string()
        } else {
            let truncated: String = self.chars().take(max_len.saturating_sub(3)).collect();
            format!("{truncated}...")
        }
    }
}

impl Stringjects for String {
    fn is_blank(&self) -> bool {
        self.as_str().is_blank()
    }
    fn truncate_ellipsis(&self, max_len: usize) -> String {
        self.as_str().truncate_ellipsis(max_len)
    }
}

/// Trait for iterators, adding a dependency-free `join`
pub trait IterJects: Iterator {
    /// Join items into a string with a separator, like itertools::join but without the dependency
    fn join(self, sep: &str) -> String
    where
        Self: Sized,
        Self::Item: Display,
    {
        let mut result = String::new();
        for (i, item) in self.enumerate() {
            if i > 0 {
                result.push_str(sep);
            }
            result.push_str(&item.to_string());
        }
        result
    }
}

impl<T: Iterator> IterJects for T {}




#[cfg(test)]
mod test;