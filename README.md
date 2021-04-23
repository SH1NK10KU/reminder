# reminder

A reminder for checking latest release in specific repository on Github.

## JSON config file

Add repository settings into `repos.json`.

```json
[
  {
    "name": "rust",
    "repository": "https://github.com/rust-lang/rust",
    "local": "",
    "status": ""
  }
]
```

## Example

* Modify config
```json
[
  {
    "name": "rust",
    "repository": "https://github.com/rust-lang/rust",
    "local": "",
    "status": ""
  },
  {
    "name": "rustup",
    "repository": "https://github.com/rust-lang/rustup",
    "local": "",
    "status": ""
  }
]
```

* Result
```shell
List of updatable repositories:
Repository {
  "name": "rust",
  "repository": "https://github.com/rust-lang/rust",
  "local": "NOT SYNC",
  "status": "1.51.0"
}
Please press enter to quit ...
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
