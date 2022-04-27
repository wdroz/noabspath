# Say no to absolute paths in your codebases!

**noabspath** search and highlight all absolute paths detected.

<p align="center"><img src="https://raw.githubusercontent.com/wdroz/noabspath/master/img/demo.gif"/></p>

## Usage

<pre>
William Droz <william.droz.ch@gmail.com>
Detect hard-coded absolute paths in codesbases

USAGE:
    noabspath [OPTIONS]

OPTIONS:
    -h, --help                         Print help information
    -i, --ignore-file <IGNORE_FILE>    File that contains patterns to ignore [default: .gitignore]
    -p, --path <PATH>                  Path of codebase to check [default: .]
    -V, --version                      Print version information

</pre>

## Use case

The most common use case is to use **noabspath** in your CI pipeline.

## Install

### Cargo

```bash
cargo install noabspath
```

### Snap

```bash
sudo snap install noabspath --edge
```