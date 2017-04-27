.PHONY: build build-c build-rust run run-c run-rust

build: build-c build-rust

build-c:
	make -C c build-all
build-rust:
	make -C rust build-all

run: run-c run-rust

run-c: build
	make -C c run-all
run-rust: build
	make -C rust run-all
