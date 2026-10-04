System gauges is a rust program that display information about your system in a linear gauge format from the inside of your terminal.

## Features
The currently supported informations about your system are : 
- your **RAM** usage
- your **SWAP** usage
- your global **CPU** usage
- your **Disk** usage

The colors and borders are customizable 

If you have more than one disk the gauges will be sized appropriately to all fit on the screen.

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
- Windows and MacOs support has not been tested but should work
- Powered by ratatui
