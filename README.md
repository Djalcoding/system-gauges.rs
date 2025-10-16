# system-gauges.rs
<img width="1051" height="563" alt="image" src="https://github.com/user-attachments/assets/a8e40aa5-234e-4efc-b57e-b8a8179214f3" />
<img width="947" height="494" alt="image" src="https://github.com/user-attachments/assets/abe73394-74e0-48c3-a60d-798744a25586" />

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

***INSTALLING FROM EXECUTABLE***
   1. *Grab the latest executable from the relase section*
   2. Add it to your path
   for bash : 
   ```bash
   echo 'export PATH=$PATH:/home/path/to/installation/directory' >> ~/.bashrc
   ```
     for zsh:
   ```zsh 
   echo 'export PATH=$PATH:/home/path/to/installation/directory' >> ~/.zshrc
   ```
      
***COMPILING FROM SOURCE***
1. *Clone the repo and cd into the directory:*
```bash
git clone https://github.com/Djalcoding/system-gauges.rs.git
cd system-gauges
```

2. *Compile the binary:* 
```bash
cargo build --release
```
3. *Add "current working directory"/target/release to path:*
   for bash : 
```bash
echo 'export PATH=$PATH:/home/path/to/current/directory/target/release' >> ~/.bashrc
```
  for zsh:
```zsh 
echo 'export PATH=$PATH:/home/path/to/current/directory/target/release' >> ~/.zshrc
```

### Prerequisites
- Rust toolchain  (recommended: use [rustup](https://rustup.rs/)) (for compiling from source)

## Usage
Help :
```bash
system-gauges -h
```
Running the programm : 
```bash
system-gauges
```

The colors are defined by your Terminal Environment.

## Notes
- The script currently as linux support as it uses termion, (it should also work on Mac-OS but it hasn't been tested)

