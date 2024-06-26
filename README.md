# Forestry

A settings manager for Linux distributions for users using window managers and not full desktop environments. Forestry uses standard terminal colors (16 color specification) for styling, making it easy to match it's appearance to your own terminal configuration.

## Motivation

I found I really enjoyed using tiling window managers after experimenting with XMonad, i3, and Qtile (XMonad being my current preference). The one thing I missed from a full desktop environment was a more centralized settings application. I decided to start on this project to help out with my daily computer usage and help organize some of my system configuration. The state of this repository will indicate if I actually managed to commit to completing this project.


## Archiecture

I want this project to be as software agnostic as possible (ex. power-profiles vs tlp) and make it easier to change system settings on the fly rather than editing a config file or opening several other software programs.

## Current Features

### Network Controls

- Ability to view, select, and join networks (`nmcli` backend)
    - No support yet for .1x networks. These networks must be setup up manually (externally). Once known, Forestry can connect to the .1x network
- Ability to select power profiles using `powerprofilesctl`
- Ability to configure displays (frontend to `xrandr`)

![Network Page](./images/network-page.png)

