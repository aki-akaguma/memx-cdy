
all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test --offline

test-no-default-features:
	cargo test --offline --no-default-features

miri:
	cargo +nightly miri test --offline

clean:
	@cargo clean
	@rm -f z.*

clippy:
	cargo clippy --offline --tests --workspace

fmt:
	cargo fmt

doc:
	cargo doc

tarpaulin:
	cargo tarpaulin --offline --engine llvm --out html --output-dir ./target

cbind:
	cbindgen --config cbindgen.toml --crate memx-cdy --output target/my_header.h
