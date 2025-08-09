# system-gauges.rs
<img width="1051" height="563" alt="image" src="https://github.com/user-attachments/assets/a8e40aa5-234e-4efc-b57e-b8a8179214f3" />
System gauges is a rust program that display information about your system in a linear gauge format from the inside of your terminal.

## Features ⚙️
The currently supported informations about your system are : 
- your **RAM** usage
- your **SWAP** uage
- your global **CPU** usage
- your **Disk** usage

The colors of the main gauges and the disk gauges are customizable.

If you have more than one disk the gauges will be sized approprietly to all fit on the screen.



## Installation

1. *Clone the repo and cd into the directory:*
```bash
git clone https://github.com/Djalcoding/system-gauges.rs.git
cd system-gauges
```

2. *Compile the binary:* 
```bash
cargo build --release
```
3. *Add the it to path:*
```bash
echo 'export PATH=$PATH:/home/path/to/current/directory/target/release' >> ~/.bashrc
```

```zsh
echo 'export PATH=$PATH:/home/path/to/current/directory/target/release' >> ~/.zshrc
```

### Note
you can also simply download the binary from the release section and add it to path.

### Prerequisites
- Rust toolchain installed (recommended: use [rustup](https://rustup.rs/))
