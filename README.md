# dropbox-dir

A cli program & lib to find your local Dropbox directory in a jiffy.

## As a CLI program

```sh
$ # print your personal directory path
$ dropbox-dir
/home/user/Dropbox
$ # you can also print your business directory path (if configured)
$ dropbox-dir --business
/home/user/Dropbox (Suff & co)
$ # you can also print both
$ dropbox-dir --all
personal: /home/user/Dropbox
business: /home/user/Dropbox (Suff & co)
```

The CLI is designed to be used easily inside scripts. To avoid calling multiple programs `dropbox-dir` can process a path given in parameter. For example you want an environnement variable containing the location of your notes.
```sh
$ export NOTES_PATH=$(dropbox-dir doc/notes)
$ echo $NOTES_PATH
/home/user/Dropbox/doc/notes
```

## As a package

```rust
extern crate dropbox_dir;

use dropbox_dir::SmartPath;

pub fn main() {
    let db_path = SmartPath::new_personal("some/dir").expect("Dropbox not configured");
    // do stuff with your now...
    println!("{}", db_path);
}
```