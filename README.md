# Peat

Peat is a command-line app to run another command-line app repeatedly with different parameters.

Peat can easily play the role of a sub-contractor in a WDL workflow to reduce scatter overhead: if you have 10,000 jobs,
but don't want to run 10,000 machines, you can use WDL to run 200 machines and use Peat to then run 50 jobs on each
machine. Peat makes it especially easy to make sure that each of the 10,000 jobs is actually run and only run once.

## Get Peat

To get the Peat source code, clone the [Peat repo](https://github.com/broadinstitute/peat):

```bash
git clone https://github.com/broadinstitute/peat.git
```

Peat is written in [Rust](https://www.rust-lang.org/). To
compile, [install the Rust toolchain](https://www.rust-lang.org/tools/install), go to the `peat` directory and compile
using:

```bash
cargo build --release
```

The peat binary will be in `target/release/peat`.

## Hello, World!

To print "Hello, world!" to the console, without repetition, we can use the file `example/hello.peat` in
the [Peat repo](https://github.com/broadinstitute/peat):

```
Peat 1.0
===
echo "Hello, World!"
```

Each Peat file has divider - the first line that contains just "===". Everything before that is the head with some
information for Peat (here, jut the version 1.0). Everything after that is the body, which is a template for a Bash
script. Here, the Bash script just contains the line with the `echo`. So, Peat will write that line to a script use Bash
to execute it, printing the desired "Hello, world".

We run this with Peat:

```bash
peat example/hello.peat
```

This produces the following output:

```
Peat file uses version 1.0
Declarations: [none]
Now evaluating
Bindings: [empty]
Hello, World!
Process completed successfully.
Done!
```

Yay, we printed, "Hello, World". 

Instead of using a file, Peat can also read from standard input, so we can, for example, run Peat by providing 
the code via a heredoc in Bash:

```
peat << EOF
Peat 1.0
===
echo "Hello, World!"
EOF
```

The body can contain more than one line, by the way:

```
peat << EOF
Peat 1.0
===
echo "Hello, World!"
echo "How are you?"
echo "Hope all is wonderful!"
EOF
```

To do something more interesting, we need to declare variables.

## Variables

Next, we look at `examples/declarations.peat`:

```
Peat 1.0
X=1
Y=2
Z=X
===
echo "Hello, declarations! X is <:X:>, Y is <:Y:> and Z is <:Z:>"
```

This declares three variables `X`, `Y`, and `Z` in the head using assignment (`=`).

In the body, there are placeholders marked with `<:` and 
`:>`, which will be filled by the values of variables, e.g. `<:X:>` will be filled by the value of `X` and so on.

We run this with

```
peat examples/declarations.peat
```

and get:

```
Peat file uses version 1.0
Declarations: X = 1, Y = 2, Z = X
Now evaluating
Bindings: X = 1, Y = 2, Z = 1
Hello, declarations! X is 1, Y is 2 and Z is 1
Process completed successfully.
Done!
```

So far, Peat has only executed one script per invocation. Next, we will let Peat iterate.

## Ranges and iterations

Next, let us look at `examples/range.peat`:

```
Peat 1.0
X = 1
Y <- 0 .. 3
===
echo "Hello, range! X is <:X:> and Y is <:Y:>"
```

Note how `Y` is followed not by an assignment operator (`=`), but by an iteration operator (`<-`).

The `0 .. 3` is a range from the lower bound (inclusive) until the upper bound (exclusive), so the range contains the 
numbers 0, 1 and 2. `Y` will be iterated over this range.

This time, peat prints:

```
Peat file uses version 1.0
Declarations: X = 1, Y <- 0 .. 3
Now evaluating
Bindings: X = 1, Y = 0
Hello, range! X is 1 and Y is 0
Process completed successfully.
Bindings: X = 1, Y = 1
Hello, range! X is 1 and Y is 1
Process completed successfully.
Bindings: X = 1, Y = 2
Hello, range! X is 1 and Y is 2
Process completed successfully.
Done!
```

We can see that the script is executed three times, with `X` always 1 and `Y` iterating over 0, 1 and 2.

We can also nest iterations, such as in `nested.peat`:

```
Peat 1.0
X <- 0 .. 5
Y <- 0 .. X
Z <- 0 .. Y
===
echo "Hello, nested! <:Z:> < <:Y:> < <:X:> < 5"
```

Here, we have three iterations. The latter ones will be nested inside the earlier ones, so we get:

```
Peat file uses version 1.0
Declarations: X <- 0 .. 5, Y <- 0 .. X, Z <- 0 .. Y
Now evaluating
Bindings: X = 2, Y = 1, Z = 0
Hello, nested! 0 < 1 < 2 < 5
Process completed successfully.
Bindings: X = 3, Y = 1, Z = 0
Hello, nested! 0 < 1 < 3 < 5
Process completed successfully.
Bindings: X = 3, Y = 2, Z = 0
Hello, nested! 0 < 2 < 3 < 5
Process completed successfully.
Bindings: X = 3, Y = 2, Z = 1
Hello, nested! 1 < 2 < 3 < 5
Process completed successfully.
Bindings: X = 4, Y = 1, Z = 0
Hello, nested! 0 < 1 < 4 < 5
Process completed successfully.
Bindings: X = 4, Y = 2, Z = 0
Hello, nested! 0 < 2 < 4 < 5
Process completed successfully.
Bindings: X = 4, Y = 2, Z = 1
Hello, nested! 1 < 2 < 4 < 5
Process completed successfully.
Bindings: X = 4, Y = 3, Z = 0
Hello, nested! 0 < 3 < 4 < 5
Process completed successfully.
Bindings: X = 4, Y = 3, Z = 1
Hello, nested! 1 < 3 < 4 < 5
Process completed successfully.
Bindings: X = 4, Y = 3, Z = 2
Hello, nested! 2 < 3 < 4 < 5
Process completed successfully.
Done!
```

We could easily tell Peat to iterate over 10,000 jobs in one call.

In the next section, we will see how to distribute the jobs into groups.

## The problem of distributing jobs 

Peat can help distribute jobs. 

Let's say we have 10,000 jobs and running them on a single machine would take too long. 

We could use the scatter feature
in WDL to distribute them over 10,000 machines, but that would be a large overhead, because we have to start up 10,000
machines, pull the Docker image 10,000 times, and copy the input files from storage to local disk 10,000 times.

The only reasonable solution might be to divide the jobs into groups. For example, we could use scatter
in WDL to fan out into 200 branches, which means 200 machines, and then run 50 jobs on each machine.

In principle, this is easy. We could write a Bash script with a loop. In practice, it is awkward and easy to get wrong, 
especially if we decide halfway in between to change the number of jobs, or the number of machines.

Peat has been designed to make this as easy as possible and blend well with WDL.

## Grouping ranges and picking a group

The next example divides a range into groups and then picks a group. It is `examples/pick.peat`:

```
Peat 1.0
N_JOBS = 10
N_GROUPS = 3
I_GROUP = 1
I <- 0 .. N_JOBS / 0 .. N_GROUPS $ I_GROUP
===
echo "This is job <:I:> of <:N_JOBS:> jobs, part of group <:I_GROUP:> of <:N_GROUPS:> groups."
```

The interesting part is the iteration of `I`. We have the range `0 .. N_JOBS` which represents the jobs, and the
range `0 .. N_GROUPS`. Then we apply the `/` operator to divide the job range by the group range. The result is a 
collection of ranges obtained by splitting the first range as evenly as possible into as many ranges as the
second range has members, and then associating each fragment with a member of the second range.

So, the job range contains ten members, 0 to 9. The group range contains three members, 0, 1 and 2. Therefore,
the jobs range is divided into three sub-ranges: 0 to 3, 4 to 6 and 7 to 9. These sub-ranges are then associated with
the members of the second range, so 0 to 3 is associated with 0, 4 to 6 is associated to 1 and 7 to 9 is associated 