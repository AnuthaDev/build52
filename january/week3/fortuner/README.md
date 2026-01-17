# fortuner: Linux kernel module that gives fortunes

Similar to fortune cookies, fortuner returns a fortune when it is read.

e.g.

```sh
$ cat /proc/fortuner
There are no stupid questions, just stupid answer.
```

## Building
Run the `make` command and load the resulting `fortuner.ko` file using `insmod`

## Implementation Notes
Uses `get_random_u32_below` function to generate a random index into the array
of fortunes.

The index is generated each time the file is opened and is maintained per file descriptor. We should not generate it per `read` call
because `proc_read` can be called multiple times per logical read.