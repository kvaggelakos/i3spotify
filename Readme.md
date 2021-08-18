# i3Spotify

This is a rust application to display spotify artist/track information in the i3bar

![Screenshot](screenshot.png?raw=true "Screenshot")

## Installation

To install using cargo:

```
cargo install i3spotify
```

Add to i3 config like below:

```
bar {
    status_command i3status | i3spotify
}
```
