# greattraits
A collection of great traits extending rust's standard library.

## Traits

For now, this collection includes only one trait, but I plan to add more in the future.

### Pathjects:
A trait for path manipulation. (only Path, but want to extend to PathBuf).
It supports several methods to open a file, delete a file, create, as well as open in bufread and bufwrite mode.


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

## License

This is my hobby project and i'm still not knowledgeable about licensing. But someone told me that MIT is a good license, so I'm using it. If you have any suggestions, please let me know.
