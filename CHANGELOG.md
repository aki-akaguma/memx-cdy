TBD
===
Unreleased changes. Release notes have not yet been written.

0.1.3 (2021-06-24)
=====

* add memx_init(): for a easy linking
* bug: "ld: error: duplicate symbol: memcpy" in the case that target is "xxx-musl".
 - for the time being, i add `#[cfg(not(target_env = "musl"))]` to the `memcpy()`.

0.1.2 (2021-06-23)
=====

* update crates: memx(0.1.13) - brush up

0.1.1 (2021-06-20)
=====

* add "staticlib" and "rlib" to crate-type
* update crates: memx(0.1.12) - the important bugs fixed

0.1.0 (2021-06-08)
=====

first commit
