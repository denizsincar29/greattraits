use std::io::{Read, Write};

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
