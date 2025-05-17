# Dora Explorer
`dora-explorer` i.e. `de` is a TUI file navigator built on top of ratatui.

It provides a **read only** experience for quick navigation over a file system. Such that you don't ever have to worry about accidentally deleting files when performing quick navigations.

It provides local file navigation support as well as GCS File path selection support.

## adding cd navigation support
to perform navigation with the dora explorer binary, paste the following function into your `~/.bashrc`

```
cde() { cd $(de $@); }
```

## How to navigate the explorer
- `hjkl` (vim left,down,up,right)
- arrow keys
- `q` to quit
- `enter` to enter a directory
- `enter` will exit and return the path to a file, if the cursor is on a file
- left and right will enter / exit a directory.
- `ctrl+h` will toggle the showing of hidden .dotfiles


## Usage
Here are some usage examples:

To open up the explorer in your CWD
```
de
```

To Open up the explorer in home directory
```
de ~
```

To Open up and navigate to the folder 
```
cde
```


To open up a gs file
```
de gs://{...}
```

To select a path and copy into your clipboard (on WSL2)
```
de | clip.exe
```

To select a path and use the selected path to open up dora 
```
de | xargs dora
```