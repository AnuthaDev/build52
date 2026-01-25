# refortuner: Linux kernel module written in Rust that gives fortunes

Similar to fortune cookies, refortuner returns a fortune when it is read.

e.g.

```sh
$ cat /dev/refortuner
There are no stupid questions, just stupid answer.
```

Based on the Rust-for-Linux Out of tree module template

## Building
Run the `make` command, with `KDIR` to the kernel tree containing Rust build metadata

```sh
make KDIR=.../linux-with-rust-support LLVM=1
```

Load the resulting `refortuner.ko` using `insmod`
