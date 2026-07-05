use std::{collections::hash_map::DefaultHasher, fmt::Display, fs::{create_dir, remove_dir_all, remove_file, File, OpenOptions}, hash::{Hash, Hasher}, io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write}, path::{Path, PathBuf}};

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
    /// Open an existing file for both reading and writing, without truncating it.
    /// Use this (instead of `create`) when you want to edit a file that already has content,
    /// e.g. with `append_line`, `insert_line`, or `remove_line`.
    fn edit(&self) -> io::Result<File>;
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
        OpenOptions::new().read(true).write(true).create(true).truncate(true).open(self)
    }
    fn edit(&self) -> io::Result<File> {
        OpenOptions::new().read(true).write(true).open(self)
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
    fn edit(&self) -> io::Result<File> {
        self.as_path().edit()
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

/// Trait for `File`, adding whole-file reads/writes, stats, and line editing
pub trait Fileject {
    /// Read the entire file into a `String`, from the start
    fn read_all_string(&self) -> io::Result<String>;
    /// Read the entire file into a `Vec<u8>`, from the start
    fn read_all_bytes(&self) -> io::Result<Vec<u8>>;
    /// Size of the file in bytes
    fn size(&self) -> io::Result<u64>;
    /// True if the file has zero bytes
    fn is_empty(&self) -> io::Result<bool>;
    /// Append raw text to the end of the file, without touching existing content
    fn append_str(&self, s: &str) -> io::Result<()>;
    /// Replace the entire content of the file with `data`
    fn overwrite_all(&self, data: &[u8]) -> io::Result<()>;
    /// Copy this file's content into another open file
    fn copy_to(&self, dest: &mut File) -> io::Result<u64>;
    /// Number of lines in the file
    fn line_count(&self) -> io::Result<usize>;
    /// Number of whitespace-separated words in the file
    fn word_count(&self) -> io::Result<usize>;
    /// A simple non-cryptographic checksum of the file's content
    fn checksum(&self) -> io::Result<u64>;
    /// Append a line (with a trailing newline) to the end of the file
    fn append_line(&self, line: &str) -> io::Result<()>;
    /// Insert a line at `index` (0-based), shifting the following lines down.
    /// If `index` is past the end, the line is appended.
    fn insert_line(&self, index: usize, line: &str) -> io::Result<()>;
    /// Remove the line at `index` (0-based), returning it if it existed
    fn remove_line(&self, index: usize) -> io::Result<Option<String>>;
}

impl Fileject for File {
    fn read_all_string(&self) -> io::Result<String> {
        let mut f = self;
        f.rewind()?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    fn read_all_bytes(&self) -> io::Result<Vec<u8>> {
        let mut f = self;
        f.rewind()?;
        let mut v = Vec::new();
        f.read_to_end(&mut v)?;
        Ok(v)
    }
    fn size(&self) -> io::Result<u64> {
        Ok(self.metadata()?.len())
    }
    fn is_empty(&self) -> io::Result<bool> {
        Ok(self.size()? == 0)
    }
    fn append_str(&self, s: &str) -> io::Result<()> {
        let mut f = self;
        f.seek(SeekFrom::End(0))?;
        f.write_all(s.as_bytes())
    }
    fn overwrite_all(&self, data: &[u8]) -> io::Result<()> {
        self.set_len(0)?;
        let mut f = self;
        f.seek(SeekFrom::Start(0))?;
        f.write_all(data)
    }
    fn copy_to(&self, dest: &mut File) -> io::Result<u64> {
        let mut f = self;
        f.rewind()?;
        io::copy(&mut f, dest)
    }
    fn line_count(&self) -> io::Result<usize> {
        Ok(self.read_all_string()?.lines().count())
    }
    fn word_count(&self) -> io::Result<usize> {
        Ok(self.read_all_string()?.split_whitespace().count())
    }
    fn checksum(&self) -> io::Result<u64> {
        let bytes = self.read_all_bytes()?;
        let mut hasher = DefaultHasher::new();
        bytes.hash(&mut hasher);
        Ok(hasher.finish())
    }
    fn append_line(&self, line: &str) -> io::Result<()> {
        let mut lines: Vec<String> = self.read_all_string()?.lines().map(String::from).collect();
        lines.push(line.to_string());
        write_lines(self, &lines)
    }
    fn insert_line(&self, index: usize, line: &str) -> io::Result<()> {
        let mut lines: Vec<String> = self.read_all_string()?.lines().map(String::from).collect();
        let idx = index.min(lines.len());
        lines.insert(idx, line.to_string());
        write_lines(self, &lines)
    }
    fn remove_line(&self, index: usize) -> io::Result<Option<String>> {
        let mut lines: Vec<String> = self.read_all_string()?.lines().map(String::from).collect();
        if index >= lines.len() {
            return Ok(None);
        }
        let removed = lines.remove(index);
        write_lines(self, &lines)?;
        Ok(Some(removed))
    }
}

/// Write `lines` back to `file`, one per line, replacing all existing content
fn write_lines(file: &File, lines: &[String]) -> io::Result<()> {
    let mut content = lines.join("\n");
    if !lines.is_empty() {
        content.push('\n');
    }
    file.overwrite_all(content.as_bytes())
}




#[cfg(test)]
mod test;