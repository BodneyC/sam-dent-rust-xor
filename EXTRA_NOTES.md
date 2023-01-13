# Ben's Extra Notes

These are really just some picky extra notes, the core of the notes are in the actual files

## README

You lack a readme, they're always useful for users:

- Rust version required
- Supporting tools required
- How to use the program or at least where to find help with that (docs/`-h`)
- Etc.

## Resources

This is just to have a resources folder for your example inputs and outputs

Not only does it keep the workspace clean, but if you want to test ten inputs with of nine edge-case plus a simple example, then having ten `.txt` files in your top level is messy 

I assume you'd have already done this if you had ten of them, but you get my point

### Output.txt

The file `output.txt` is kinda transient, its content changes depending on what input you last gave it

Unless you want that one file to have lots of pointless changes in lots of commits, I'd either add it to your `.gitignore` file

Or, to keep the file but stop observing its changes in git:

```sh
git update-index --assume-unchanged output.txt
```
