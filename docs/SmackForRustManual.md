# Installing, running and using the tool
## Prerequisites
### Vagrant
You can determine if Vagrant is already installed by typing
`$ vagrant`
in a shell.

If Vagrant is not installed, then installation instructions can be found at https://www.vagrantup.com/docs/installation/.

This will allow you to use our included Vagrant file to automate the building and installation of the tool in a virtual machine.
### git
If git is not currently installed, follow the instructions at https://git-scm.com/book/en/v2/Getting-Started-Installing-Git.

## Download and install the tool
Select a working directory and run
```
$ git clone https://github.com/smackers/smack.git
$ cd smack
$ git checkout rust-prelims
$ vagrant up
```

At this point Vagrant will take about 20 minutes to build and install SMACK in the virtual machine.

## Entering the virtual machine
Once Vagrant finishes building the virtual machine you can type 
```
$ vagrant up
```
to start the virtual machine, and
```
$ vagrant ssh
```
to enter the virtual machine.

To shutdown the virtual machine type
```$ vagrant halt``` after logging out of the virtual machine.
