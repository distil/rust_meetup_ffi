.PHONY: build-all build-calling_c build-calling_rust build-memory_management build-ownership build-asynchronous_callbacks run-all run-calling_rust run-ownership

build-all: build-calling_c build-calling_rust build-memory_management build-ownership build-asynchronous_callbacks

run-all: run-calling_rust run-ownership

bin:
	mkdir -p bin
lib:
	mkdir -p lib

build-calling_c: src/calling_c.c lib
	cc -fPIC src/calling_c.c -Wall -shared -o lib/libcalling_c.so

build-calling_rust: src/calling_rust.c bin
	cc src/calling_rust.c -L../rust/calling_rust/target/debug/ -lcalling_rust -Wall -o bin/calling_rust

build-memory_management: src/memory_management.c lib
	cc -fPIC src/memory_management.c -Wall -shared -o lib/libmemory_management.so

build-ownership: src/ownership.c bin
	cc src/ownership.c -L../rust/ownership/target/debug/ -lownership -Wall -o bin/ownership

build-asynchronous_callbacks: src/asynchronous_callbacks.cpp lib
	c++ src/asynchronous_callbacks.cpp -std=c++11 -Wall -shared -o lib/libasynchronous_callbacks.so

run-calling_rust: build-calling_rust
	bin/calling_rust

run-ownership: build-ownership
	bin/ownership
