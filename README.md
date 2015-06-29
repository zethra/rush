Rusty
=====

What is Rusty?
--------------
Rusty is a command line shell written in the Rust Language. Due to the nature
of the language used it offers fast execution and memory safety for the shell
itself (no guarantees about the programs you run with it.)

At this point in time the program is far from being functional in an every day
environment.

###Requirements for Rusty

'''
Rust Nightly
Cargo
ctags
[rusty-tags](https://github.com/dan-t/rusty-tags)
'''

I use vim and have not generated any emacs tags. In the future I will so that
other devs may use it.

###The Current Version is 0.0.4 meaning Rusty can and has support for:

- [x] Execute programs entered into the command line
- [x] Change Directory
- [x] Pipes
- [x] ctags (with the use of [rusty-tags](https://github.com/dan-t/rusty-tags))

###In Progress
- [ ] Prompts and customization

###Planned but not implemented:
- [ ] cd - functionality
- [ ] Auto Completion
- [ ] Better output formatting for completed commands
- [ ] Scripting language and processor.
- [ ] Output errors on stderr automagically
- [ ] Shell logic
- [ ] Calculator utility for fun. Likely to be on back burner

###Code Cleanup Ideas
- [ ] Docs. So many Docs
- [ ] Reduce need for clone() and work on proper ownership
- [ ] Format code in a uniform way
- [ ] Reduce let bindings
- [ ] Reduce need of Vec<&str> as inputs

Probably more. Will add as ideas come to mind.

###Rust Version
Currently using 1.2 Nightly due to need of unstable features. Has not been
tested with any other version.

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
```

###Contact
Email: mgattozzi@gmail.com
Twitter:  [@mgattozzi](https://twitter.com/mgattozzi)
