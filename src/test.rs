use std::io::{Read, Write};
use std::path::PathBuf;

use super::*;

#[test]
fn writefile() {
    let txt=Path::new("file.txt");
    let mut f=txt.create().expect("cannot create file");
    write!(f, "hello!").expect("cannot write file");
    txt.delete().expect("can't delete!");
}

#[test]
fn readfile() {
    let txt=Path::new(".gitignore");  // gitignore file is always there!
    let mut f=txt.open().expect("can't open file!");
    let mut string=String::new();
    f.read_to_string(&mut string).expect("cannot read to string!");
    assert!(!string.is_empty());
}

#[test]
fn create_delete_dir(){
    let p=Path::new("./testdir");
    p.mkdir().expect("cannot mkdir!");
    assert!(p.exists());
    let mut f=p.join("hello.txt").create().expect("can't create file!");
    write!(f, "hello world! this is test!").expect("can't write file!");
    drop(f); // dropped because we'll delete the folder!
    assert!(p.join("hello.txt").exists());
    p.delete().expect("can't delete folder!");
    assert!(!p.exists());
}

#[test]
fn pathbuf_works() {
    let p = PathBuf::from("./testdir_pathbuf");
    p.mkdir().expect("cannot mkdir!");
    assert!(p.exists());
    p.delete().expect("can't delete folder!");
    assert!(!p.exists());
}

#[test]
fn is_blank() {
    assert!("".is_blank());
    assert!("   \t\n".is_blank());
    assert!(!"hi".is_blank());
    assert!(!String::from("  hi  ").is_blank());
}

#[test]
fn truncate_ellipsis() {
    assert_eq!("hello".truncate_ellipsis(10), "hello");
    assert_eq!("hello world".truncate_ellipsis(8), "hello...");
    assert_eq!("hi".truncate_ellipsis(2), "hi");
}

#[test]
fn iter_join() {
    let v = vec![1, 2, 3];
    assert_eq!(v.into_iter().join(", "), "1, 2, 3");
    let empty: Vec<i32> = vec![];
    assert_eq!(empty.into_iter().join(", "), "");
}

#[test]
fn search_finds_text() {
    let txt = Path::new("search_test.txt");
    let mut f = txt.create().expect("cannot create file");
    write!(f, "hello world, hello rust").expect("cannot write file");
    drop(f);
    assert_eq!(txt.search("world").unwrap(), Some(6));
    assert_eq!(txt.search("nope").unwrap(), None);
    txt.delete().expect("can't delete!");
}

#[test]
fn search_bin_finds_bytes() {
    let bin = Path::new("search_bin_test.bin");
    let mut f = bin.create().expect("cannot create file");
    f.write_all(&[0x00, 0x01, 0xFF, 0x02, 0x03]).expect("cannot write file");
    drop(f);
    assert_eq!(bin.search_bin(&[0xFF, 0x02]).unwrap(), Some(2));
    assert_eq!(bin.search_bin(&[0x05, 0x06]).unwrap(), None);
    bin.delete().expect("can't delete!");
}

#[test]
fn grep_finds_lines() {
    let txt = Path::new("grep_test.txt");
    let mut f = txt.create().expect("cannot create file");
    write!(f, "first line\nsecond needle line\nthird needle needle line").expect("cannot write file");
    drop(f);
    let matches = txt.grep("needle").unwrap();
    assert_eq!(matches.len(), 3);
    assert_eq!(matches[0], Line { line_number: 2, text: "second needle line".to_string(), matched: "needle".to_string(), start: 7, end: 13 });
    assert_eq!(matches[1].line_number, 3);
    assert_eq!(matches[1].start, 6);
    assert_eq!(matches[2].line_number, 3);
    assert_eq!(matches[2].start, 13);
    txt.delete().expect("can't delete!");
}
