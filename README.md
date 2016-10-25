# Rush - The Rust Shell
[![Build Status](https://travis-ci.org/zethra/rush.svg?branch=master)](https://travis-ci.org/zethra/rush)
[![Build status](https://ci.appveyor.com/api/projects/status/rfg5y8nbskuj1w42/branch/master?svg=true)](https://ci.appveyor.com/project/zethra/rush/branch/master)

**Warning:** Rush is still in development and is not feature complete or stable.

A fork of [Rusty](https://github.com/mgattozzi/Rusty)

###Features
- [x] Single command execution
- [x] Perciststant history
- [x] Pipes
- [x] File redirection
- [x] Quote parsing

###TODO
- [ ] Automatic config generation
- [ ] Logical Operators
- [ ] Stdout and Stderr redirect
- [ ] Tab Completion
- [ ] Command colorization
- [ ] Command based environment variables
- [ ] Command based aliases
- [ ] Bash script support
- [ ] Script based config

###Rust Version
Currently using 1.14 Nightly. Has not been tested with any other version.


###Install
- Clone this repo 
- Build using cargo
- Copy config/rust.toml to ~/.rush.toml

###Config File
Rusty uses [toml](https://github.com/toml-lang/toml) to customize it's use.
Here are some of the configuration options:

####Prompts
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

####Aliases
The [alias] is required in the config file but anything underneath is optional.
Below is an example of how it would look in your file in order to use them:
```
[alias]
gpm = "git push master"
gc = "git commit"
ls = "ls -al"
```

####Enivronment Variables
The [env_var] is required in the config file but anything underneath is optional.
Below is an example of how it would look in your file in order to use them:
```
[env_var]
EDITOR = "vim"

```

###Contributing
If you'd like to contribute to the project please sumbit a pull request.  Help is very appreciated.