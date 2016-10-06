# Rush - The Rust Shell

A fork of [Rusty](https://github.com/mgattozzi/Rusty)

###TODO
- [ ] File redirection
- [ ] Tab Completion
- [ ] Environment variables
- [ ] Bash script support
- [ ] Script based config

###Rust Version
Currently using 1.14 Nightly. Has not been tested with any other version.

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
michael@flame ~/ %
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