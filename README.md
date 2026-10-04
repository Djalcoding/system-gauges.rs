# system-gauges.rs
<img width="1051" height="563" alt="image" src="https://github.com/user-attachments/assets/a8e40aa5-234e-4efc-b57e-b8a8179214f3" />
<img width="947" height="494" alt="image" src="https://github.com/user-attachments/assets/abe73394-74e0-48c3-a60d-798744a25586" />

System gauges is a rust program that display information about your system in a linear gauge format from the inside of your terminal.

## Features
The currently supported informations about your system are : 
- your **RAM** usage
- your **SWAP** usage
- your global **CPU** usage
- your **Disk** usage

The colors and borders are customizable 

If you have more than one disk the gauges will be sized appropriately to all fit on the screen.

## Installation

***INSTALLING FROM Crates.io***
```bash
    cargo install system-gauges
```

### Prerequisites
- Cargo

## Usage
Help :
```bash
system-gauges -help
```
Running the program : 
```bash
system-gauges
```

## Notes
Windows and MacOs support has not been tested but is expected to work


Powered by ratatui
