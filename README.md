Rusty
=====
[![BuildStatus](https://travis-ci.org/mgattozzi/Rusty.svg?branch=master)](https://travis-ci.org/mgattozzi/Rusty)
[![Coverage
Status](https://coveralls.io/repos/mgattozzi/Rusty/badge.svg?branch=master&service=github)](https://coveralls.io/github/mgattozzi/Rusty?branch=master)
[![Join the chat at https://gitter.im/mgattozzi/Rusty](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/mgattozzi/Rusty?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)
[![IssueStats](http://www.issuestats.com/github/mgattozzi/rusty/badge/pr)](http://www.issuestats.com/github/mgattozzi/rusty)
[![IssueStats](http://www.issuestats.com/github/mgattozzi/rusty/badge/issue)](http://www.issuestats.com/github/mgattozzi/rusty)

What is Rusty?
--------------
Rusty is a command line shell written in the Rust Language. Due to the nature
of the language used it offers fast execution and memory safety for the shell
itself (no guarantees about the programs you run with it.)

At this point in time the program is far from being functional in an every day
environment.

A lot of development has gone on since my last version bump and with it
I can say the base level functionality is complete. Now I'm starting to
add the features that actually make a shell worth while to use.
Currently Rusty is undergoing a huge refactoring of the code base to
make it easier to navigate as well as readable. Moving away from the use
of Vec<&str> and using String as the method of dealing with things upon
entry has made the code more readable. Though the lower level functions
in the code still require the use of the Vec<&str> type.

Rusty's library is called Rush (might be separated in the future so
others can use it without Rusty) and is built to manipulate the terminal
and execute commands for the user.

###Documentation
The docs for rush can be found [here](https://mgattozzi.github.io/Rusty/rush/).
It's only in the initial stages but will be more fleshed out as time
goes on.

While much of the process submodule is exposed for now eventually the
only method that will be able to be called is
rush::process::execute::interpret
As this will handle all the logic and everything else required with
executing commands and can be used to execute quite complex commands
within a rust program without having to worry about how to implement it.

###Requirements for Rusty

- Rust Nightly
- Cargo

####Optional
- ctags
- [rusty-tags](https://github.com/dan-t/rusty-tags)

###The Current Version is 0.1.3 meaning Rusty can and has support for:

- [x] Execute programs entered into the command line
- [x] Change Directory
- [x] Pipes
- [x] ctags (with the use of [rusty-tags](https://github.com/dan-t/rusty-tags))
- [x] Left and Right Arrow Key movement
- [x] Backspace support

###In Progress
- [ ] Configuration
	- [x] Access config file in ~/.rusty.toml else default config file
	- [x] Prompt
		- [x] Update cwd
		- [x] Update prompt
		- [x] Parse config files
	- [x] Aliases
	- [x] Environment Variables
		- [x] Set enviornment variables
		- [x] Be able to add to current variable e.g. PATH = "PATH:/home/user/.bin"
	- [ ] Source config files
- [ ] Command History
	- [x] Store Commands in History Buffer
	- [ ] Save History to file
	- [ ] Load History upon start
	- [ ] Access History to run previous commands
- [ ] Buffered Input
	- [ ] Keyboard Interupts
		- [ ] Implement all keys
		- [x] Handle Interupts
	- [x] Take in one char at a time
	- [x] Buffer inputs
	- [ ] Terminal Manipulation
		- [x] Left and right cursor movement
		- [x] Backspace
- [ ] Finish Interpret
	- [ ] Shell Logic
		- [x] xor
		- [x] and
		- [ ] nand
		- [ ] not
		- [x] or
	- [x] pipes
	- [ ] Parallel Commands
	- [x] Single Command Execution
	- [ ] Execute Commands by logic, precedence, and order

I'm also in the middle of refactoring a lot of the code and splitting it
into smaller submodules and grouping functions better, so as to make
rush more viable and make code easier to maintain.

I also just switched to using copperline for getting input from the
command line and it's written in Rust meaning it's way safer than what
I was attempting to do.

###Planned but not implemented:
- [ ] Better error handling using try!()
- [ ] cd - functionality
- [ ] Directory stack e.g. dirs 2 moves to second directory on the stack
- [ ] File redirection
- [ ] Auto Completion
- [ ] Better output formatting for completed commands
- [ ] Scripting language and processor.
- [ ] Output errors on stderr automagically
- [ ] Calculator utility for fun. Likely to be on back burner
- [ ] Makefile to compile and install automatically
- [ ] Pluggable modules that others write

###Known Issues
- Super user does not seem to be working for some reason at this point

###Code Cleanup Ideas
- [ ] Docs. So many Docs
- [ ] Reduce need for clone() and work on proper ownership
- [ ] Format code in a uniform way
- [ ] Reduce let bindings
- [ ] Get rid of unsafe or make super safe
	- [ ] Make sure char conversions work properly

###Rust Version
Currently using 1.4 Nightly due to need of unstable features. Has not been
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

###Testing and Execution
You can run the inbuilt tests to determine if the functions are working as
intended with the command

```
cargo test
```

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

####Getting Involved
Clone the repository and open up a pull request for merging a new
feature that you've implemented.

I also need people just use and abuse the shell. For now it only works
with VT100 Terminal Code and I don't know if it works on other types of
shells. Also finding exploits and the like are always welcome.

Open up an issue on Github if you find something wrong with the use of
the shell or find an exploit or something like that. Since Rusty isn't
for daily use public disclosure is fine for security flaws and will help
solicit design feedback regarding those kinds of issues.

###Contact and verification
Email: mgattozzi@gmail.com

Twitter:  [@mgattozzi](https://twitter.com/mgattozzi)

Keybase: [mgattozzi](https://keybase.io/mgattozzi)
Commits are now signed and the key can be verified at the above link

###Donations
If you want to donate (not required this project will always be free) to the project you can use my bitcoin address:
1HJm93qp2625SEuq2gFxjzV558c6F4gKCq

