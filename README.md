# Rush - The Rust Shell
[![Build Status](https://travis-ci.org/zethra/rush.svg?branch=master)](https://travis-ci.org/zethra/rush)
[![Build status](https://ci.appveyor.com/api/projects/status/rfg5y8nbskuj1w42/branch/master?svg=true)](https://ci.appveyor.com/project/zethra/rush/branch/master)

**Warning:** Rush is still in development and is not feature complete or stable.

###News
After taking a short break from working on rush I decieded to rewrite the parser using a peg parser.


### Features
- [x] Single command execution
- [x] Persistent history
- [x] Pipes
- [x] File redirection
- [x] Quote parsing
- [x] Evironment variables

### TODO
- [ ] Script based config (like every other shell)
- [ ] Full POSIX support
- [ ] Windows and Mac support
- [ ] Tab Completion
- [ ] Command colorization

### Usage
- Requires rust nightly-2017-01-06
- Clone this repo 
- Build using cargo
- Copy config/rust.toml to ~/.rush.toml

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