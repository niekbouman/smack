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
```
$ vagrant halt
```
after logging out of the virtual machine.

# Our benchmark suite
Our feature-test benchmark suite can be found at https://github.com/smackers/smack/tree/rust-prelims/examples/rust. It is also
present in the virtual machine in `examples/rust`. The suite can be easily run by changing into this directory and running
```
./regtest.py
```
Note that the entire benchmark run will take about an hour, depending on hardware.

## An overview of SMACK extensions
In order to use the full functionality of our extensions, Rust programs should start with:
```rust
#[macro_use]
mod smack;
use smack::*;
```

**Note: Run `smack program.rs` first in order for SMACK to create the `smack` crate.**

This will bring in definitions for our modeled `Vec` and `Box` classes, as well as macros for `assert` and `assume`.
Additionally, all primitive integer types and bool have the _NonDet_ trait implemented. This means one can write
```rust
let x = 5u8.nondet();
```
When compiled by the Rust compiler directly, `x` will have the value `5`, however when run in SMACK, `x` will be nondeterministic,
and in this example it can take any value between 0 and 255 inclusive as this is an unsigned, 8-bit integer.

We can add contraints to `x` by writing
```rust
assume!(x < 30);
```

In SMACK this means that `x` is now in 0 to 29 inclusive, and translates into a noop outside of SMACK.

Putting this together, we can run SMACK on the following, small program `example1.rs`:
```rust
// example1.rs
#[macro_use]
mod smack;
use smack::*;
fn main() {
  let x = 5u8.nondet();
  assume!(x < 30);
  assert!(x*x < 29*29);
}
```
We run check the example in SMACK by running
```
$ smack example1.rs
```
SMACK will report an error on this program and its backtrace will show that the assertion can fail if `x == 29`. We can fix the example by changing the assertion to `assert!(x*x <= 29*29);`, which SMACK will report as "verified".

We can also check a program for integer overflow in `example2.rs`:
```rust
// example2.rs
#[macro_use]
mod smack;
use smack::*;
fn main() {
  let x = 65u8.nondet();
  let y = x*x;
}
```
If we compile and run this program using:
```
$ rustc example2.rs
$ ./example2
```
The program will report a panic due to integer overflow, since `65*65 > 255` and overflow checking is turned on by default when the Rust compiler is run in debug mode.

In order to get SMACK to report the integer overflow error, we use the `--integer-overflow` flag to enable checking:
```
$ smack --integer-overflow example2.rs
```
SMACK will report that an integer overflow can happen in the program, and gives an example value for `x` for when this is possible.

We can produce a variation of this program, `example2a.rs`:
```rust
// example2a.rs
#[macro_use]
mod smack;
use smack::*;
fn main() {
  let x = 63u8.nondet();
  assume!(x < 64);
  let y = x*x;
}
```
When we run
```
$ smack --integer-overflow example2a.rs
```
there is no report of an integer overflow as `x` is now less than 64.
### Dynamic memory
We can create dynamically sized arrays using the `Vec` class and heap allocated memory using the `Box` class. For example
```rust
let x = Box::new(6);
```
will create a pointer to a heap allocated number, and
```rust
let x = vec![1,2,3,4,5,6];
```
will create a heap allocated dynamic array with initial contents 1,2,...,6. Dereferencing of a Box as in
```rust
let mut x = Box::new(7):
*x = 8;
assert!(*x == 8);
```
is supported. We support indexing, growing and and iterating over a `Vec` as in:
```rust
// vec_example1.rs
#[macro_use]
mod smack;
use smack::*;

fn main() {
  let mut x = vec![1,1,2,3];
  x.push(4);
  x.push(5);
  x.push(6);
  x[0] = 0;
  for &v in x {
    assert!(*v < 7);
  }

  for i in 0..x.len() {
    assert!(x[i] == i);
  }
}
```

# A more complex example
```rust
// example3.rs
#[macro_use]
mod smack;
use smack::*;
fn square_vec(x: &Vec<u8>) -> Vec<u8> {
  let mut result = Vec::new();
  for &v in x {
    result.push(v*v);
  }
  result
}

fn main() {
  let a = 0u8.nondet();
  let b = 1u8.nondet();
  let c = 2u8.nondet();
  let d = 3u8.nondet();
  assume!(a < 64 && b < 64 && c < 64 && d < 64);
  let x = vec![a,b,c,d,e];
  let squared_x = square_vec(&x);
  assert!(x.len() == squared_x.len());
  for i in 0..x.len() {
    assert!(x[i]*x[i] == squared_x[i]);
  }
}
```

In this example, we call a function which squares the contents of our vector. We check that the returned vector is the same length as the original vector.
And each entry in the squared vector is the square of the corresponding entry in the original vector. We also constrain the values
in the vector to prevent integer overflow. To check this example in SMACK we run
```
$ smack --no-memory-splitting --unroll=5 example3.rs
```
which will check the program for assertion violations. For this example, we need to specify a loop unroll bound, which needs to be at least 5 here, which is indicated by `--unroll=5`.Bugs can be inserted into the program to make sure SMACK is working properly.
We can also check for integer overflow by running
```
$ smack --no-memory-splitting --unroll=5 --integer-overflow example3.rs
```
This should return no errors as well, but we can seed the program with an error by changing the bounds on `a, b, c` or `d` in the assume statement.

Each benchmark category is represented by a directory within `examples/rust`. Each file is annotated with the SMACK flags needed for verification, as well as the expected result of verification.

# General information on SMACK options
+ `--bit-precise`: This flag should be used if the program contains any bit-wise operations. Note that this option can slow verification of the program significantly.
+ `--no-memory-splitting`: This flag is often necessary when veryfing more complex Rust programs as SMACK doesn't understand Rust's memory aliasing.
+ `--unroll=n`: This controls the loop and recursion bound. It is best to set this to a higher value to permit a more thorough
+ `--time-limit=n`: `n` is the number of seconds to let SMACK run.
