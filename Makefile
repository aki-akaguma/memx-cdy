
all: README.md

README.md: src/lib.rs
	cargo readme > $@

clean:
	@rm -fr target

cbind:
	cbindgen --config cbindgen.toml --crate memx-cdy --output target/my_header.h
