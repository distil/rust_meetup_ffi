.PHONY: build build-c build-rust run run-c run-rust

build:
	make -C c bin lib
	make -C c lib/libcalling_c.so
	make -C rust build-calling_c
	make -C rust build-calling_rust
	make -C c bin/calling_rust
	make -C c lib/libmemory_management.so
	make -C rust build-memory_management
	make -C rust build-ownership
	make -C c bin/ownership
	make -C c lib/libasynchronous_callbacks.so
	make -C rust build-asynchronous_callbacks

run: run-c run-rust

run-c: build
	make -C c run
run-rust: build
	make -C rust run

clean: clean-c clean-rust

clean-c:
	make -C c clean
clean-rust:
	make -C rust clean
