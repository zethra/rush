# Rush - The Rust Shell
[![Build Status](https://travis-ci.org/zethra/rush.svg?branch=master)](https://travis-ci.org/zethra/rush)

**Warning:** Rush is still in development and is not feature complete or stable.

### News
Rush is not under active development.  I may continue work on it at some point but there aren't enough hours in the day.


### Features
- [x] Single command execution
- [x] Persistent history
- [x] Pipes
- [x] Quote parsing
- [x] Evironment variables
- [x] Script based config
- [x] File name completion

### Planned Features
- [ ] File redirection (partly done)
- [ ] Job control commands (fg, bg, etc.)
- [ ] Full POSIX support
- [ ] Full command completion
- [ ] Command colorization

### Posible Features
- [ ] Windows and Mac support

### Usage
- Built on rust nightly-2017-02-21
- Clone this repo 
- Build using cargo
- Copy config/rushrc.sh to ~/.rushrc

### Config File
Rusty uses [toml](https://github.com/toml-lang/toml) to customize it's use.
Here are some of the configuration options:

#### Prompts
```
[prompt]
left = "%U@%H %L %R"
```
The characters following the % can all be used or not. Here is what they stand
for:
- %U = Current user
- %H = Hostname
- %L = Current Working Directory
- %R = Whether you are root or not

They can be arranged in any order you desire. Any extra characters in the field
will just be put in the prompt.
Following the above example for my computer the prompt would look like:
```
zethra@linux-box ~/ $
```

#### Aliases
The [alias] is required in the config file but anything underneath is optional.
Below is an example of how it would look in your file in order to use them:
```
[alias]
gpm = "git push master"
gc = "git commit"
ls = "ls -al"
```

#### Enivronment Variables
The [env_var] is required in the config file but anything underneath is optional.
Below is an example of how it would look in your file in order to use them:
```
[env_var]
EDITOR = "vim"
```

### Inspiration
Rush was orininally a fork of [Rusty](https://github.com/mgattozzi/Rusty) although I've changed a lot since then.
My peg grammar was largly taken from [js-shell-parse](https://github.com/grncdr/js-shell-parse).

### Contributing
If you'd like to contribute to the project please submit a pull request.  Help is very appreciated.
