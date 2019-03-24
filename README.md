# maze

Rust implementation of [Mazes for Programmers](https://pragprog.com/book/jbmaze/mazes-for-programmers) by [Jamis Buck](https://github.com/jamis)

## Example

To generate a maze, `cargo run` inside the `./cli` directory.

```
+---+---+---+---+---+---+---+---+---+---+
|                                       |
+---+---+---+---+   +---+---+   +   +   +
|                   |           |   |   |
+   +---+   +   +---+   +   +---+   +   +
|   |       |   |       |   |       |   |
+   +   +---+   +---+   +   +---+   +   +
|   |   |       |       |   |       |   |
+   +   +   +---+---+   +   +   +---+   +
|   |   |   |           |   |   |       |
+   +---+---+   +   +---+   +   +---+   +
|   |           |   |       |   |       |
+---+---+---+---+---+---+   +---+---+   +
|                           |           |
+---+   +   +   +---+---+---+   +---+   +
|       |   |   |               |       |
+---+---+---+   +   +   +   +   +---+   +
|               |   |   |   |   |       |
+---+   +---+   +---+   +   +---+---+   +
|       |       |       |   |           |
+---+---+---+---+---+---+---+---+---+---+
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
