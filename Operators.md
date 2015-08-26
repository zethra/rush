#Operators
The following is a list of Operators that you can use in your commands
on the command line:

###To Be Implemented
- +=+ will cause the commands to the left and right of it run in
  parallel. e.g. ls / | sort | uniq +=+ date would run ls / | uniq on one
  thread and date on another thread
- && will run the first command and if that works then it runs the second
  command
- || will run the first command, if it executes the second one is not run otherwise the second one is run
- |x| will run both commands.only if one is succesful
