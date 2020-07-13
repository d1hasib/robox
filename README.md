# Robox
```bash
(Greetings, Human!)
  o
   o
    +-----+
  O-| o o |-O
    | +++ |
    +-----+
```
Robox is a talking robot. The robot with feelings! Though you are the one who will determine Robox mood.

## Installation 

Run this command in the termianl- 
```bash 
cargo install --git https://github.com/d1hasib/robox
```
If you haven't do it already, add the following line in your
`.bashrc` or `.zshrc` file.
```
export PATH=$PATH:$HOME/.cargo/bin
```
This will help `shell` to find `robox`

## Usage 

For default result-
```bash 
robox "Greetings, Human!"
```
To set a mood-
```bash
robox --mood {mood}
```
Available **moods**-  

- happy
- sad
- angry
- wonder
- crazy
- wink
- dead
- pirate

To make message colorful-
```bash
robox --color {color}
```
Available **colors**-

- black
- red
- green
- yellow
- blue
- magenta
- cyan
- white

For more information- 
```bash
robox --help
```
