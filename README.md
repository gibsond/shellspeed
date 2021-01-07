## Shellspeed

**Shellspeed** is an application that attempts to indicate the performance of a shell by measuring the speed that a set of specified or default commands take to execute.

Currently, the best way to run this application is to allow it to run its default set of commands.  You specify the shell you want to test, and the net time to run these commands will be returned:

./shellspeed -s ion -d

You can also specify a specific command to run:
./shellspeed -s ion -c "ls"

However, because one command can take such a short time, often you will se a net time of negative due to the applications subtracting from a baseline of invoking the shell with no commands.

## Shellspeed application help

The command line help can be seen by typing:

./shellspeed -h

resulting in:

Welcome to Shellspeed

Shellspeed Help:
This app measures the speed of a set of specified or default commands in a specified shell.

 It does this by sending a stream of shell script commands using the -c switch in the shell command.

 Since not all shell commands allow loops, at this time, it creates a stream of repetative shell commands

 starting out with let i=0 for ion, and i=0 for nu and all others, then:



 ion:               "; let k=$((i+1)); let amt=$((k*i))"

 nu and all others: "; k=$((i+1)); amt=$((k*i))"



 It averages 10 calls of the shell with no script commands, and 10 calls of the shell with the script above, then subtracts the

 averages to get a net measurement of how long it takes to run the script.


eg: 


shellspeed -h  for help
shellspeed --help  for help
shellspeed -s ion -d  to measure the time to execute a default shell script: 
shellspeed -s ion -c "ls" or "for i in {1..10000}; do { let k=$((i+1)); let amt=$((k*i)); } done;" for example to measure the time to execute the ls command in the ion shell.

