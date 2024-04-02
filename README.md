## Description
A small command-line application to display slides from the terminal written in Rust. 

mtwcl means:
- Manuel
- Tailwind
- cover
- letter

## Installation

### Automatic (recommended)

```bash
curl -o- https://raw.githubusercontent.com/hombrew/mtwcl/main/scripts/install.sh | bash
```

Currently only Mac and Linux are supported.

### From source

Installing from source requires a local [Rust environment](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/hombrew/mtwcl.git

cd mtwcl
cargo install --path .
```

## Usage

```bash
mtwcl
```

Use `left` and `right` keys to navigate through the slides. Press `Esc` to exit.
