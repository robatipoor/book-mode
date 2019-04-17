# book-mode

A flexible tool for keeping track of what you read, when you read, how
you read, all while being a hand-writable text format.

`book-mode` is a CLI. It might at some point also be an emacs mode,
depending on how passionately I feel about that (PRs welcome 😬)

## Building

`book-mode` is written in Rust and can be built with [`cargo`].

```console
$ cargo build --release
```

[`cargo`]: https://github.com/rust-lang/cargo

## Spec

`book-mode` fundamentally only parses text files. These can either be
written by you, the user, or at a later point also the CLI. Each `.books`
file contains several sections that use indentation to differentiate between
keys and values in a very simple way. Headers use special indentation that's
obviously distinguishable from entries. Here's an example.

```
Some Title 					2017
    Genre: Science Fiction
    ISBN: 1234-56789-123
    Pages: 420
  
Another Title					2017-2018
    Genre: Political fiction
    ISBN: 4321-98765-321
    Pages: 666
```

As you can see `book-mode` can also take into account that some books will be
started in one year and finished in another. When generating graphs, books
that span multiple years will be counted as 3/4 for both years, to make the
graphs look as natural as possible. For raw numbers, it counts as 1 for both
years because a book was indeed read, just not finished in time.

## License 

`book-mode` is free software, licensed under the "GPL-3" or later.
