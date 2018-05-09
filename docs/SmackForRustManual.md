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

To shutdown the virtual machine type,
```$ vagrant halt``` after logging out of the virtual machine.

## An overview of SMACK extensions
In order to use the full functionality of our extensions, Rust programs should start with:
```#[macro_use]
mod smack;
use smack::*;
```

Note: Run `smack program.rs` first in order for SMACK to create the `smack` crate.

This will bring in definitions for our modeled `Vec` and `Box` classes, as well as macros for `assert` and `assume`.
Additionally, all primitive integer types and bool have the _NonDet_ trait implemented. This means one can write
```let x = 5u8.nondet();```
When compiled by the Rust compiler directly, `x` will have the value `5`, however when run in SMACK, `x` will be nondeterministic,
and in this exapmle it can take any value between 0 and 255 inclusive as this is an unsigned, 8-bit integer.

We can add contraints to `x` by writing
```assume!(x < 30);```

In SMACK this means that `x` is now in 0 to 29 inclusive, and translates into a noop outside of SMACK.

Putting this together, we can run SMACK on the following, small program `example1.rs`:
```fn main() {
  let x = 5u8.nondet();
  assume!(x < 30);
  assert!(x*x < 29*29);
}
```
We run check the example in SMACK by running
```$ smack example1.rs
```
SMACK will report an error on this program and its backtrace will show that the assertion can fail if `x == 29`. We can fix the example by changing the assertion to `assert!(x*x <= 29*29` which SMACK will report as verified.

