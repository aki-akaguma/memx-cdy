
all:

clean:

cbind:
	cbindgen --config cbindgen.toml --crate memx-cdy --output target/my_header.h
