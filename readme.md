# greattraits
A collection of great traits extending rust's standard library.

## Traits

### Pathjects:
A trait for path manipulation. Implemented for both `Path` and `PathBuf`.
It supports several methods to open a file, delete a file, create, as well as open in bufread and bufwrite mode.

### Stringjects:
A trait for `str` and `String`. Adds `is_blank()` (empty or only whitespace) and `truncate_ellipsis(max_len)` (truncate to at most `max_len` chars, appending "..." if it had to cut).

### IterJects:
A trait for any `Iterator`. Adds `join(sep)`, a dependency-free version of `itertools::join`, so you don't need to pull in itertools just for that.


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

## License

This is my hobby project and i'm still not knowledgeable about licensing. But someone told me that MIT is a good license, so I'm using it. If you have any suggestions, please let me know.
