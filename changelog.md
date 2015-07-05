<a name="0.0.4"></a>
## 0.0.4 (2015-07-05)


#### Features

* **$calc**  utils/calc, readme -Started a little work on calc util. Updated README. ([7fde1ce1](https://github.com/mgattozzi/rusty/commit/7fde1ce15b64a67a87b4bfb0bb9a2b9bdb511348))
* **$cat**  utils/cat -Began working on a cat util. -Planning on porting all utils to rust for usage ([2be0c6f1](https://github.com/mgattozzi/rusty/commit/2be0c6f123ea204f0894bd022d092b051f50c042))
* **$cd**
  *  /utils/cd, Tilde Expansion -cd works as intended with ~ expansion. Error handling included. ([a4fdb01b](https://github.com/mgattozzi/rusty/commit/a4fdb01b99f642750c809304541f0e7d5343dd76))
  *  cd, test fix -Tests fixed to accept new inputs to functions -cd is in a working state ([84cd3185](https://github.com/mgattozzi/rusty/commit/84cd318559df800c8f2d6942898bcf51e14b3dd0))
* **$ctags**  rusty-tags.vi -Added ctags support for rusty with https://github.com/dan-t/rusty-tags ([33ef39a3](https://github.com/mgattozzi/rusty/commit/33ef39a33ba83248a7f7c87655985c1cd46936c3))
* **$execution**  core/execute Managed to make execute work as planned: - Possible instability with execute method coercing Vector to &str type - As such rusty should only be compiled against rust nightly from now   until only stable features are used. - Also created tests for execute method. ([eb3c8e7c](https://github.com/mgattozzi/rusty/commit/eb3c8e7cb612d790699f9ced6a80259f67c34d7e))
* **$idea**  utils/calc -Added file for adding calculator abilities to the command line in the  future ([539631d1](https://github.com/mgattozzi/rusty/commit/539631d1ac34e39b87747b6c3c532564966f9f6e))
* **$initial**  github -Initial commit and addition of LICENSE ([d1e3cc8e](https://github.com/mgattozzi/rusty/commit/d1e3cc8e54dfa9f49896baac42294a70182f9f4f))
* **$logic**
  *  core/logic, readme -Completed or function in logic module. -Updated readme. ([f71e9f14](https://github.com/mgattozzi/rusty/commit/f71e9f14f5b8c82bca8880da8260af4d449a9221))
  *  core/logic -and function for logic works as intended. -Effort will need to be made for parsing && in input ([2f8fd34f](https://github.com/mgattozzi/rusty/commit/2f8fd34fbb7f6cefff7e5ccba5538d869f55ed84))
* **$pipes**
  *  core/execute -Piping now works using the | in the typing of a command. -Further efforts to work on it's error outputting as well as halting  execution if given a bad command will commence ([1385ff98](https://github.com/mgattozzi/rusty/commit/1385ff988ee96af1721cab2cef3fa39e5e553db7))
  *  core/execute -Work continues on the pipes. The logic for piped is in place I just  need to get execute_pipe to work with an input of sorts ([51ffd146](https://github.com/mgattozzi/rusty/commit/51ffd14625f5205afc8d2b4764edfd47b5bfcf4d))
  *  core/execute -piped function now successfully calls itself and splits inputs into  the proper programs. -Now just need to find a way for program to take output of the first  command and input it into the second command ([bf5812ce](https://github.com/mgattozzi/rusty/commit/bf5812ce809b8319d1db70449024ba5a35e1237b))
  *  core/execute -Can detect if input has pipes. -e.g.: ps auxx|grep firefox and ps auxx | grep firefox are both valid. -Work now is to begin processing the pipes themselves and how to execute the commands ([925d95c8](https://github.com/mgattozzi/rusty/commit/925d95c8db67c99b3ebbeafdeecb59d63a35eaf9))
* **$plugin**  rust-clippy -Added rust-clippy linter for better code writing ([250f7056](https://github.com/mgattozzi/rusty/commit/250f70568f271df6805a61fe467c8b8147e01b84))
* **$prompt**
  *  core/prompt -Added dynamic prompt that reads from config file located in config  folder. -Work can now go on to getting the value from a file in ~/.rusty.toml first ([98015037](https://github.com/mgattozzi/rusty/commit/980150375c7bf58643ec0a860aa3102f2a9892d6))
  *  core/prompt -Prompt now internally updates cwd with each call to cd. -If the cd call goes to anywhere in the user's home directory then  Prompt shortens /home/user to ~/ . -This required an unstable feature in 1.3 Rust Nightly and has been added to lib.rs ([b00782e3](https://github.com/mgattozzi/rusty/commit/b00782e3c331a2803859b0e8b607e59ba70bf8bf))
  *  core/prompt -Work on prompt now puts it on the command line succesfully. -Work needs to be done about parsing config file and dynamically  updating prompt if things change ([7fa61f95](https://github.com/mgattozzi/rusty/commit/7fa61f95a4d4f3238aa2c928fdf2fe8c6a1f7c35))
  *  core/prompt.rs -Began work on prompt ([5ebf8f70](https://github.com/mgattozzi/rusty/commit/5ebf8f707347f01e645778dd1c9701f7b1bd86e5))
* **$stderr**  /core/execute, Wrapper Function -Output of stderr automatically on failure has been implemented. -Interpret wrapper function is beginning to be developed. Pipes are next -All other functions besides interpret have been made private.  Only interpret is needed now. ([1a447f02](https://github.com/mgattozzi/rusty/commit/1a447f027e5d4b60c8300bfae4289b31762aea53))
* **$travis-ci**
  *  travis.yml -Updated travis.yml to upload results to coveralls. This is a test of that. ([74080e2e](https://github.com/mgattozzi/rusty/commit/74080e2e15a7a545901d8f5a393e9dafe2d6e1af))
  *  travis.yml -Added .travis.yml ([a2b8e209](https://github.com/mgattozzi/rusty/commit/a2b8e20933a1507a2390fbbf1cab39e19a48ba33))
* **config**  core/config read_in_config attempts to open file in home then default file. Panics otherwise. ([bde8ac68](https://github.com/mgattozzi/rusty/commit/bde8ac68dee8c296c733c2afd7a2bae170fd9e08))

#### Bug Fixes

* **$pipes**  core/execute -Fixed || crashing program, added better error handling for pipes.  Definitely could use expansion on the subject. ([7d00b49c](https://github.com/mgattozzi/rusty/commit/7d00b49cd33d9bc6a8be214cda45c9548edd3b1b))
* **$tests**  core/execute, core/logic -Fixed tests for execute module. -Removed crappy code from logic module. Should be passed commands to do  logic on. -I'll have to make execute even more robust with splitting all kinds of things ([12fd8620](https://github.com/mgattozzi/rusty/commit/12fd86206b7ec8de72c861313cdd4f825263aa7a))



