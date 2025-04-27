# WebCrane
The old project name is wsvcs (WebSockets Version Control System)
  
## Description
This project aims to resolve git's commit size limitation. The idea is to work with chunkified files, thus minimizing memory and network usage.

## Getting Started

### Dependencies
- rust 1.86
- UPnP support

### Installing
```bash
git clone https://github.com/encore-ecosystem/webcrane.git
cd webcrane
cargo build --release
```
### Executing program
```
webcrane --<option>
options:
- init
- push
- pull <code>
```

## Version History
* 0.1
    * Initial Release

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## ToDo:
- Fix file hash calculation
- Add chunk validation
- Logging
- Project validation
- Support for merging
- Support for branching and committing
