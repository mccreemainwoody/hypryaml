# hypryaml

hypryaml is a simple CLI tool to parse and load dynamic configurations for the
Hypr tools family using YAML files. It is made in Rust, but also possesses a
[Python implementation](https://github.com/mccreemainwoody/hyprthemes), which
can also be used as a package to use within other Python applications.

This is a very, very simple package which is still in development. It will
(probably) receive much improvements in the future. But for now, it is usable
if you want to use it in higher-level projects for pretty cool stuff. :)

This package may be used for, for example :

- Dynamic theme selectors (called from the terminal or even a waybar!)
- Profil selectors
- Automatic theme switchers (combined with CRON tasks, custom script...)
- Anything else you see fit !

## Dependencies

### During build

- rustc and cargo
- That's all!

### On usage

- A running Hyprland instance
- hyprctl *(hypryaml sends message to the Hypr tools through the hyprctl CLI)*

## Installation

Build and install the package on your local machine using cargo :

```{bash}
cargo install --git ssh://git@github.com/mccreemainwoody/hypryaml.git
```

If Cargo cannot fetch the repository on its own, you can also download the
package by yourself and let cargo install it afterwards :

```{bash}
git clone git@github.com/mccreemainwoody/hypryaml.git
cargo install --path hypryaml
```

Then, don't forget to add Cargo to your PATH to make sure you can access the
compiled binary !

```{bash}
export PATH="~/.cargo/bin:$PATH"
```

## Usage

First, create a configuration file. Using your favorite editor, open a new
file and create a YAML node following the structure :

```{yaml}
# my_config.yaml

hyprland:
    general:
        border_size: 1
        col.inactive_border: rgb(ff0000) rgb(ffff00) 45rad
        col.active_border: rgb(33ccff) rgb(00ff99) 45rad
```

*(In the case of Hyprland, each leaf node will be parsed as a configuration
  keyword to apply. For example, the node on line 3 (if correctly placed
  anywhere wihtin the `hyprland` node) will be understood as "the keyword
  `general:col.inactive_border` must be set to the value `rgb(ff0000)
  rgb(ffff00) 45rad`". All the available Hyprland keywords can be changed
  through this structure.)*

Then, call the CLI with the file as a parameter :

```{bash}
hypryaml apply my_config.yaml
```

And voilà! Your configuration is automatically loaded into your current
Hyprland environment instance.

## Improvements

For now, the project is only in the state of a usable prototype: only Hyprland
keywords are supported as of now *(which is still sufficient to start creating
themes!)*. Planned or potential improvement may include :

- Support for hyprpaper
- Support for hyprsunset
- Support for static or file-based configurations (CRON task, convert a YAML
  configuration to a module file config)
- Revert the last applied configuration and rollback on error
- Permanently save configuration attributes
- Improve Rust code quality and performance

If you wish to contribute to the project, do not hesitate to open a pull
request or even a feature or bug issue ! Specific improvements may take time to
arrive unless requested, and the project is still in a very early state. As
such, anyone can bring their own piece to this project concept if they want
to !
