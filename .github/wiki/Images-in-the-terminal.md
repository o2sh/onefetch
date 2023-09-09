Onefetch supports displaying images using [`kitty`](https://sw.kovidgoyal.net/kitty/graphics-protocol.html), [`Sixel`](https://en.wikipedia.org/wiki/Sixel) and [`iTerm`](https://www.iterm2.com/documentation-images.html).

<p align="center">
<img src="https://raw.githubusercontent.com/o2sh/onefetch/main/assets/screenshot-1.png" height="250px">
</p>

When running `onefetch --image ./My-picture.jpg`, the program looks for the first `Image Backend` supported by the terminal and use it to display the requested image instead of the Ascii logo.

If you decide to go manual, and want to force the use of a specific image backend --> `onefetch --image ./My-picture.jpg --image-backend sixel|kitty|iterm`

### Sixel

The Sixel protocol is handled by multiple terminal emulators such as [`xterm`](https://invisible-island.net/xterm/) (enabled via flag `-ti 340`), [`mlterm`](https://wiki.ubuntu.com/Mlterm) and [`WezTerm`](https://github.com/wez/wezterm) which are actively maintained.

You can increase the color resolution using the `--color-resolution` flag.

### Kitty

The kitty terminal graphics protocol used on the terminal of the same name allows the program running in the terminal, to render graphics to the screen of the terminal emulator.

### ITerm

The iTerm inline image protocol supported by iTerm2 (also WezTerm) allows to display images within the terminal.