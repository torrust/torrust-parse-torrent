# Torrust Torrent File Parser

This is a sample command line application that parses a torrent file and prints
out the information in the torrent file.

It reuses code from the [Torrust Index Backend](https://github.com/torrust/torrust-index-backend) project.

It's not intended to be used in production. It was created to find a bug described here:

<https://github.com/torrust/torrust-index-backend/issues/266>

Modules follow the structure of the original project.

## Usage

```s
cargo run ./torrents/mandelbrot_set_01.torrent
```
