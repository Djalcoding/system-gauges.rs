
System gauges is a rust program that display information about your system in a linear gauge format from the inside of your terminal.

## Features
The currently supported informations about your system are : 
- your **RAM** usage
- your **SWAP** usage
- your global **CPU** usage
- your **Disk** usage

The colors of the main gauges and the disk gauges are customizable.

If you have more than one disk the gauges will be sized appropriately to all fit on the screen.

## Installation
```bash
cargo install system-gauges
```
### Note
This should not be installed as a library.

## Usage
Help : (this will display all arguments along with colors)
```bash
system-gauges -h
```
Running the program : 
```bash
system-gauges
```
Setting a custom color : 
```bash
system-gauges -c blue
```
Setting a custom color for disks: 
```bash
system-gauges -d red
```
The colors are defined by your Terminal Environment.

## Notes
- The script currently only has Linux support as it uses termion, (it should also work on Mac-OS but it hasn't been tested)

