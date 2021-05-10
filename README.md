# Peat

Peat is a command-line app to run another command-line app of your choice repeatedly with different parameters.

Peat can easily play the role of a sub-contractor in a WDL workflow to reduce scatter overhead: if you have 10,000 jobs,
but don't want to run 10,000 machines, you can use WDL to run 200 machines and use Peat to then run 50 jobs on each
machine. Peat makes it especially easy to make sure that each of the 10,000 jobs is actually run and only run once.

## Get Peat

To get the Peat source code, clone the [Peat repo](https://github.com/broadinstitute/peat):

```shell
git clone https://github.com/broadinstitute/peat.git
cd peat
```

To use Version 1.0.0:

```shell
git checkout v1.0.0
```

Peat is written in [Rust](https://www.rust-lang.org/). To
compile, [install the Rust toolchain](https://www.rust-lang.org/tools/install), go to the `peat` directory and compile
using:

```shell
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

Each Peat file has divider - the first line that contains just `===`. Everything before that is the head, which starts
with the version line `Peat 1.0`, followed optionally by declarations, one per line (more later). Everything after the
divider is the body, which is a template for an `sh` script. Here, the script just contains the line with the `echo`
. So, Peat will write that line to a script and then use `sh` to execute it, printing the desired "Hello, world".

We run this with Peat:

```shell
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

Instead of using a file, Peat can also read from standard input, so we can, for example, run Peat by providing the code
via a heredoc in `sh`:

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
`:>`, which will be filled by the values of variables, e.g. `<:X:>` will be filled by the value of `X` (i.e. 1)
and so on.

Declared variables can be used in subsequent declarations, such as `X` has been used in the declaration of `Z`.

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

One may wonder why in the output, `X` does not start at 0? Actually, `X` does start at 0, but for `X=0`, the range for
`Y` is empty. Likewise, for `X=1`, `Y` can only be `Y=0`, and then the range for `Z` is empty. To get a non-empty range
for `Z`, `X` needs to be at least 2.

We could easily tell Peat to iterate over 10,000 jobs in one call.

If that is too much for one call, we will see how to distribute the jobs into groups in the next sections.

## Why distribute jobs into groups?

Peat can help distribute jobs.

Let's say we have 10,000 jobs and running them on a single machine would take too long.

We could use the scatter feature in WDL to distribute them over 10,000 machines, but that would be a large overhead,
because we have to start up 10,000 machines, pull the Docker image 10,000 times, and copy the input files from storage
to local disk 10,000 times.

The only reasonable solution might be to divide the jobs into groups. For example, we could use scatter in WDL to fan
out into 200 branches, which means 200 machines, and then run 50 jobs on each machine.

In principle, this is easy. We could write a shell script with a loop. In practice, it is awkward and easy to get wrong,
especially if we decide at some later point to change the number of jobs, or the number of machines.

Peat has been designed to make this as easy as possible and blend well with WDL. The next section explains, how.

## Grouping ranges and picking a group

The next example divides a range into groups and then picks a group. It is `examples/pick1.peat`:

```
Peat 1.0
I <- 0 .. 10 / 0 .. 3 $ 1
===
echo "This is job <:I:> of 10 jobs, part of group 1 of 3 groups."
```

This gives the following output:

```
Peat file uses version 1.0
Declarations: I <- 0 .. 10 / 0 .. 3 $ 1
Now evaluating
Bindings: I = 4
This is job 4 of 10 jobs, part of group 1 of 3 groups.
Process completed successfully.
Bindings: I = 5
This is job 5 of 10 jobs, part of group 1 of 3 groups.
Process completed successfully.
Bindings: I = 6
This is job 6 of 10 jobs, part of group 1 of 3 groups.
Process completed successfully.
Done!
```

The interesting part is the iteration of `I`.

The expression `0 .. 10 / 0 .. 3 $ 1` divides the range of jobs into three sub-ranges with indices 0, 1 and 2, and then
pick the sub-range with index 1 for `I` to iterate over, resulting in iteration over 4, 5 and 6.

Let us break this down into details.

The range `0 .. 10` consists ot the numbers { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 }.

The range `0 .. 3` consists of the numbers { 0, 1, 2 }.

The range division `0 .. 10 / 0 .. 3` takes the first range and divides it into sub-ranges, with one sub-range for each
member of the second range. The result is { 0: { 0, 1, 2, 3 }, 1: { 4, 5, 6 }, 2: { 7, 8, 9 } }.

Then we apply the pick operator `$` which picks the subrange of the given index, here 1, so the result is the numbers 4,
5 and 6, or written as range, `4 .. 7`.

To get the same results, but be more flexible and more self-commenting, we replace constants by variables
in `examples/pick2.peat`. By using variables, we also make it more easy to embed into a larger context, such as WDL:

```
Peat 1.0
N_JOBS = 10
N_GROUPS = 3
I_GROUP = 1
I <- 0 .. N_JOBS / 0 .. N_GROUPS $ I_GROUP
===
echo "This is job <:I:> of <:N_JOBS:> jobs, part of group <:I_GROUP:> of <:N_GROUPS:> groups."
```

Finally, if we wanted to iterate over all groups, we could do so, as in `examples/pickall.peat`:

```
Peat 1.0
N_JOBS = 10
N_GROUPS = 3
I_GROUP <- 0 .. N_GROUPS
I <- 0 .. N_JOBS / 0 .. N_GROUPS $ I_GROUP
===
echo "This is job <:I:> of <:N_JOBS:> jobs, part of group <:I_GROUP:> of <:N_GROUPS:> groups."
```

This will iterate over all groups, and therefore over all jobs:

```
Peat file uses version 1.0
Declarations: N_JOBS = 10, N_GROUPS = 3, I_GROUP <- 0 .. N_GROUPS, I <- 0 .. N_JOBS / 0 .. N_GROUPS $ I_GROUP
Now evaluating
Bindings: N_JOBS = 10, N_GROUPS = 3, I_GROUP = 0, I = 0
This is job 0 of 10 jobs, part of group 0 of 3 groups.
Process completed successfully.
Bindings: N_JOBS = 10, N_GROUPS = 3, I_GROUP = 0, I = 1
This is job 1 of 10 jobs, part of group 0 of 3 groups.
Process completed successfully.
Bindings: N_JOBS = 10, N_GROUPS = 3, I_GROUP = 0, I = 2
This is job 2 of 10 jobs, part of group 0 of 3 groups.
Process completed successfully.
Bindings: N_JOBS = 10, N_GROUPS = 3, I_GROUP = 0, I = 3
This is job 3 of 10 jobs, part of group 0 of 3 groups.
Process completed successfully.
Bindings: N_JOBS = 10, N_GROUPS = 3, I_GROUP = 1, I = 4
This is job 4 of 10 jobs, part of group 1 of 3 groups.
Process completed successfully.
Bindings: N_JOBS = 10, N_GROUPS = 3, I_GROUP = 1, I = 5
This is job 5 of 10 jobs, part of group 1 of 3 groups.
Process completed successfully.
Bindings: N_JOBS = 10, N_GROUPS = 3, I_GROUP = 1, I = 6
This is job 6 of 10 jobs, part of group 1 of 3 groups.
Process completed successfully.
Bindings: N_JOBS = 10, N_GROUPS = 3, I_GROUP = 2, I = 7
This is job 7 of 10 jobs, part of group 2 of 3 groups.
Process completed successfully.
Bindings: N_JOBS = 10, N_GROUPS = 3, I_GROUP = 2, I = 8
This is job 8 of 10 jobs, part of group 2 of 3 groups.
Process completed successfully.
Bindings: N_JOBS = 10, N_GROUPS = 3, I_GROUP = 2, I = 9
This is job 9 of 10 jobs, part of group 2 of 3 groups.
Process completed successfully.
Done!
```

## Docker images

Peat 1.0.0 is available as Docker image for Alpine and Ubuntu:

- Alpine 3.13: `gcr.io/nitrogenase-docker/peat:1.0.0-alpine`
- Ubuntu 21.04: `gcr.io/nitrogenase-docker/peat:1.0.0-ubuntu`

Other images may be available upon request, or create take inspiration from
the [Dockerfiles in Peat repo](https://github.com/broadinstitute/peat/tree/main/docker) to make your own.
