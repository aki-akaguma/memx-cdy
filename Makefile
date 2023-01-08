
all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

clean:
	@rm -fr target

cbind:
	cbindgen --config cbindgen.toml --crate memx-cdy --output target/my_header.h
