# WebCrane

## Description

This project aims to resolve git's commit size limitation. The idea is to work with chunkified files, thus minimizing memory and network usage.

## Getting Started

### Dependencies

- rust 1.86
- UPnP support

### Installing

```bash
git clone https://github.com/encore-ecosystem/WebCrane.git
cd WebCrane
cargo build --release
```

### Executing program

```sh
webcrane --<option>
options:
- init
- push
- pull <code>
```

## Version History

* 0.1
  - Initial Release

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## ToDo

- Logging
- Project validation
- Support for merging
- Support for branching and committing
