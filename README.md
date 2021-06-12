# memx-cdy
c interface of memory functions like a libc memcmp(), memchr(), memmem(), memcpy(), memset()

## Features

* Rewriting with rust lang.
* Faster os libc.

## Quick install

1. you can install this into cargo bin path:

```text
cargo install memx-cdy
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.

# Usage
Easy to use, LD_PRELOAD.

1. usecase: cargo install
```text
$ LD_PRELOAD=~/.cargo/bin/libmemx_cdy command
```

or

```text
$ export LD_PRELOAD=~/.cargo/bin/libmemx_cdy
```

2. usecase: debian package

```text
$ LD_PRELOAD=/usr/lib/libmemx_cdy.0.1 command
```

or

```text
$ export LD_PRELOAD=/usr/lib/libmemx_cdy.0.1
```

# References

- [`memx`](https://crates.io/crates/memx)

# Benchmark Results

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  371.160 us |  175.900 us |  368.700 us |  174.850 us |
| memx_memchr             |  217.110 us |   65.294 us |  225.590 us |   63.192 us |
| memx_memchr_basic       |  231.240 us |   82.630 us |  236.100 us |   84.101 us |
| memx_memchr_libc        |  291.380 us |   83.695 us |  625.620 us |  208.250 us |
| memchr_memchr           |  224.570 us |   65.662 us |  231.470 us |   64.060 us |
| memx_cdy_memchr         |  224.970 us |   69.979 us |  226.700 us |   68.712 us |
