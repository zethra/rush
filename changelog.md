<a name=""></a>
##  (2015-07-03)


#### Features

* **$calc**  utils/calc, readme -Started a little work on calc util. Updated README. ((7fde1ce1))
* **$cat**  utils/cat -Began working on a cat util. -Planning on porting all utils to rust for usage ((2be0c6f1))
* **$cd**
  *  /utils/cd, Tilde Expansion -cd works as intended with ~ expansion. Error handling included. ((a4fdb01b))
  *  cd, test fix -Tests fixed to accept new inputs to functions -cd is in a working state ((84cd3185))
* **$ctags**  rusty-tags.vi -Added ctags support for rusty with https://github.com/dan-t/rusty-tags ((33ef39a3))
* **$execution**  core/execute Managed to make execute work as planned: - Possible instability with execute method coercing Vector to &str type - As such rusty should only be compiled against rust nightly from now   until only stable features are used. - Also created tests for execute method. ((eb3c8e7c))
* **$idea**  utils/calc -Added file for adding calculator abilities to the command line in the  future ((539631d1))
* **$initial**  github -Initial commit and addition of LICENSE ((d1e3cc8e))
* **$logic**
  *  core/logic, readme -Completed or function in logic module. -Updated readme. ((f71e9f14))
  *  core/logic -and function for logic works as intended. -Effort will need to be made for parsing && in input ((2f8fd34f))
* **$pipes**
  *  core/execute -Piping now works using the | in the typing of a command. -Further efforts to work on it's error outputting as well as halting  execution if given a bad command will commence ((1385ff98))
  *  core/execute -Work continues on the pipes. The logic for piped is in place I just  need to get execute_pipe to work with an input of sorts ((51ffd146))
  *  core/execute -piped function now successfully calls itself and splits inputs into  the proper programs. -Now just need to find a way for program to take output of the first  command and input it into the second command ((bf5812ce))
  *  core/execute -Can detect if input has pipes. -e.g.: ps auxx|grep firefox and ps auxx | grep firefox are both valid. -Work now is to begin processing the pipes themselves and how to execute the commands ((925d95c8))
* **$plugin**  rust-clippy -Added rust-clippy linter for better code writing ((250f7056))
* **$prompt**
  *  core/prompt -Added dynamic prompt that reads from config file located in config  folder. -Work can now go on to getting the value from a file in ~/.rusty.toml first ((98015037))
  *  core/prompt -Prompt now internally updates cwd with each call to cd. -If the cd call goes to anywhere in the user's home directory then  Prompt shortens /home/user to ~/ . -This required an unstable feature in 1.3 Rust Nightly and has been added to lib.rs ((b00782e3))
  *  core/prompt -Work on prompt now puts it on the command line succesfully. -Work needs to be done about parsing config file and dynamically  updating prompt if things change ((7fa61f95))
  *  core/prompt.rs -Began work on prompt ((5ebf8f70))
* **$stderr**  /core/execute, Wrapper Function -Output of stderr automatically on failure has been implemented. -Interpret wrapper function is beginning to be developed. Pipes are next -All other functions besides interpret have been made private.  Only interpret is needed now. ((1a447f02))
* **$travis-ci**
  *  travis.yml -Updated travis.yml to upload results to coveralls. This is a test of that. ((74080e2e))
  *  travis.yml -Added .travis.yml ((a2b8e209))

#### Bug Fixes

* **$pipes**  core/execute -Fixed || crashing program, added better error handling for pipes.  Definitely could use expansion on the subject. ((7d00b49c))
* **$tests**  core/execute, core/logic -Fixed tests for execute module. -Removed crappy code from logic module. Should be passed commands to do  logic on. -I'll have to make execute even more robust with splitting all kinds of things ((12fd8620))



