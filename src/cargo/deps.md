# Dependencies

Most programs have dependencies on some libraries. If you have ever managed
dependencies by hand, you know how much of a pain this. Luckily, the Rust
ecosystem comes standard with `cargo`! `cargo` can manage dependcies for a
project.

To create a new Rust project,

```sh
# A binary
cargo new --bin foo

# OR A library
cargo new foo
```

For the rest of this chapter, I will assume we are making a binary, rather than
a library, but all of the concepts are the same.

After the above commands, you should see something like this:

```txt
foo
├── Cargo.toml
└── src
    └── main.rs
```

The `main.rs` is the root source file for your new project -- nothing new there.
The `Cargo.toml` is the config file for `cargo` for this project (`foo`). If you
look inside it, you should see something like this:

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
```

You can read more extensively about all of the available configuration options
[here](http://doc.crates.io/manifest.html).

The `name` field under `package` determines the name of the project. This is
used by `crates.io` if you publish the crate (more later). It is also the name
of the output binary when you compile.

The `version` field is a crate version number using [Semantic
Versioning](http://semver.org/).

The `authors` field is a list of authors used when publishing the crate.

The `dependencies` section lets you add a dependency for your project.

For example, suppose that I want my program to have a great CLI. You can find
lots of great packages on [crates.io](https://crates.io) (the official Rust
package registry). One popular choice is [clap](https://crates.io/crates/clap).
As of this writing, the most recent published version of `clap` is `2.27.1`. To
add a dependency to our program, we can simply add the following to our
`Cargo.toml` under `dependencies`: `clap = "2.27.1"`.  And of course, `extern
crate clap` in `main.rs`, just like normal. And that's it! You can start using
`clap` in your program.

`cargo` also supports other types of dependencies. Here is just a small
sampling. You can find out more
[here](http://doc.crates.io/specifying-dependencies.html).

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # from crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
bar = { path = "../bar" } # from a path in the local filesystem
```

To build our project we can execute `cargo build` anywhere in the project
directory (including subdirectories!). We can also do `cargo run` to build and
run. Notice that these commands will resolve all dependencies, download crates
if needed, and build everything, including your crate. (Note that it only
rebuilds what it has not already built, similar to `make`).

Voila! That's all there is too it!
