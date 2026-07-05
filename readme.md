# greattraits
A collection of great traits extending rust's standard library.

## Traits

### Pathjects:
A trait for path manipulation. Implemented for both `Path` and `PathBuf`.
It supports several methods to open a file, delete a file, create, as well as open in bufread and bufwrite mode.
`create()` now opens the file for both reading and writing (truncating it if it already exists), so it works directly with `Fileject` methods below. `edit()` opens an existing file for reading and writing *without* truncating it, for when you want to modify a file that already has content.
It also adds `search(needle)` (find a substring in a text file, returns byte index), `search_bin(needle)` (find a byte sequence in a binary file, returns byte index), and `grep(needle)` (find a substring line by line, returns a `Vec<Line>` with the line number, the full line text, the matched text, and its start/end column).

### Stringjects:
A trait for `str` and `String`. Adds `is_blank()` (empty or only whitespace) and `truncate_ellipsis(max_len)` (truncate to at most `max_len` chars, appending "..." if it had to cut).

### IterJects:
A trait for any `Iterator`. Adds `join(sep)`, a dependency-free version of `itertools::join`, so you don't need to pull in itertools just for that.

### Fileject:
A trait for `File` itself (not just paths). Requires the file to have been opened for both reading and writing, e.g. via `Pathjects::create` or `Pathjects::edit`.
- `read_all_string()` / `read_all_bytes()` - read the whole file from the start, no need to make a buffer yourself
- `size()` / `is_empty()` - file size in bytes, and whether it's zero
- `append_str(s)` - append raw text to the end without touching existing content
- `overwrite_all(data)` - replace the entire file content
- `copy_to(&mut other_file)` - copy this file's content into another open file
- `line_count()` / `word_count()` - count lines / whitespace-separated words
- `checksum()` - a simple non-cryptographic hash of the content, for quick "did this file change" checks
- `append_line(line)` - append a line (with trailing newline) to the end
- `insert_line(index, line)` - insert a line at a 0-based index, shifting the rest down
- `remove_line(index)` - remove the line at a 0-based index, returning it if it existed


## Usage

Use a simple shell command to install the crate:

```shell
cargo add greattraits
```

Then, you can use the trait in your code:

```rust
use greattraits::Pathjects;

fn main(){
    let folder = Path::new("myfolder");
    folder.mkdir().expect("cannot create folder");
    let txt=folder.join("hello.txt");
    let mut f=txt.create().expect("cannot create file");
    write!(f, "hello!").expect("cannot write file");
    txt.delete().expect("can't delete!");
    folder.delete().expect("can't delete folder!");
}
```

`Stringjects` and `IterJects` in action:

```rust
use greattraits::{Stringjects, IterJects};

fn main() {
    assert!("   ".is_blank());
    assert_eq!("hello world".truncate_ellipsis(8), "hello...");
    let joined = vec![1, 2, 3].into_iter().join(", ");
    assert_eq!(joined, "1, 2, 3");
}
```

Searching files:

```rust
use greattraits::Pathjects;
use std::path::Path;

fn main() {
    let f = Path::new("myfile.txt");
    if let Some(idx) = f.search("needle").unwrap() {
        println!("found at byte {idx}");
    }
    for line in f.grep("needle").unwrap() {
        println!("{}:{}-{}: {}", line.line_number, line.start, line.end, line.text);
    }
}
```

Editing a file with `Fileject`:

```rust
use greattraits::{Pathjects, Fileject};
use std::path::Path;

fn main() {
    let p = Path::new("notes.txt");
    let f = p.create().expect("cannot create file"); // read+write, truncated

    f.append_line("first note").unwrap();
    f.append_line("third note").unwrap();
    f.insert_line(1, "second note").unwrap();
    // file now contains: "first note\nsecond note\nthird note\n"

    println!("{} lines, {} words", f.line_count().unwrap(), f.word_count().unwrap());

    let removed = f.remove_line(0).unwrap();
    println!("removed: {removed:?}");

    p.delete().expect("can't delete!");
}
```

## License

This is my hobby project and i'm still not knowledgeable about licensing. But someone told me that MIT is a good license, so I'm using it. If you have any suggestions, please let me know.
