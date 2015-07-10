Rusty
=====
![Image of Travis CI Build Status]
(https://travis-ci.org/mgattozzi/Rusty.svg?branch=master)

What is Rusty?
--------------
Rusty is a command line shell written in the Rust Language. Due to the nature
of the language used it offers fast execution and memory safety for the shell
itself (no guarantees about the programs you run with it.)

At this point in time the program is far from being functional in an every day
environment.

###Requirements for Rusty

- Rust Nightly
- Cargo
- ctags
- [rusty-tags](https://github.com/dan-t/rusty-tags)

###The Current Version is 0.0.4 meaning Rusty can and has support for:

- [x] Execute programs entered into the command line
- [x] Change Directory
- [x] Pipes
- [x] ctags (with the use of [rusty-tags](https://github.com/dan-t/rusty-tags))

###In Progress
- [ ] Configuration
	- [x] Access config file in ~/.rusty.toml else default config file
	- [x] Prompt
		- [x] Update cwd
		- [x] Update prompt
		- [x] Parse config files
	- [x] Aliases
	- [ ] Environment Variables
		- [x] Set enviornment variables
		- [ ] Be able to add to current variable e.g. PATH = "PATH:/home/user/.bin"
	- [ ] Source config files

###Planned but not implemented:
- [ ] cd - functionality
- [ ] Directory stack e.g. dirs 2 moves to second directory on the stack
- [ ] Command History
- [ ] File redirection
- [ ] Auto Completion
- [ ] Better output formatting for completed commands
- [ ] Scripting language and processor.
- [ ] Output errors on stderr automagically
- [ ] Shell logic
- [ ] Calculator utility for fun. Likely to be on back burner
- [ ] Makefile to compile and install automatically
- [ ] Pluggable modules that others write

###Code Cleanup Ideas
- [ ] Docs. So many Docs
- [ ] Reduce need for clone() and work on proper ownership
- [ ] Format code in a uniform way
- [ ] Reduce let bindings

Probably more. Will add as ideas come to mind.

###Rust Version
Currently using 1.2 Nightly due to need of unstable features. Has not been
tested with any other version.

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
Side note setting of the path variable with the current value plus whatever
else is not currently supported but will be in the future.
So PATH = "PATH:/home/user/.bin" won't work

###Testing and Execution
You can run the inbuilt tests to determine if the functions are working as
intended with the command

```
cargo test
```

To test for memory leaks of the program install valgrind and run

```
cargo build
valgrind target/debug/rusty
```

By not executing programs and just pressing enter a few times you'll loop
through Rusty's code to test it for memory leaks. Then type exit. If other
programs are executed they can introduce their errors into the test.

You can run the program with

```
cargo run
```

If you want an executable to run on a daily basis (not reccomended currently)
then run the following:

```
cargo build --release
cp target/release/rusty /destination/in/PATH
chsh user /path/to/rusty/executable
cp /path/to/rusty/config/rusty.toml ~/.rusty.toml
```

###Contact
Email: mgattozzi@gmail.com

Twitter:  [@mgattozzi](https://twitter.com/mgattozzi)
