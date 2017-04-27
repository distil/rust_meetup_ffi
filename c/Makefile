.PHONY: run run-calling_rust run-ownership clean clean-calling_c clean-calling_rust clean-memory_management clean-ownership clean-asynchronous_callbacks

run: run-calling_rust run-ownership

clean: clean-calling_c clean-calling_rust clean-memory_management clean-ownership clean-asynchronous_callbacks

bin:
	mkdir -p bin
lib:
	mkdir -p lib

lib/libcalling_c.so: src/calling_c.c
	cc -fPIC src/calling_c.c -Wall -shared -o lib/libcalling_c.so

bin/calling_rust: src/calling_rust.c
	cc src/calling_rust.c -L../rust/calling_rust/target/debug/ -lcalling_rust -Wall -o bin/calling_rust

lib/libmemory_management.so: src/memory_management.c
	cc -fPIC src/memory_management.c -Wall -shared -o lib/libmemory_management.so

bin/ownership: src/ownership.c
	cc src/ownership.c -L../rust/ownership/target/debug/ -lownership -Wall -o bin/ownership

lib/libasynchronous_callbacks.so: src/asynchronous_callbacks.cpp
	c++ src/asynchronous_callbacks.cpp -fPIC -std=c++11 -Wall -shared -o lib/libasynchronous_callbacks.so

run-calling_rust: bin/calling_rust
	LD_LIBRARY_PATH=../rust/calling_rust/target/debug bin/calling_rust

run-ownership: bin/ownership
	LD_LIBRARY_PATH=../rust/ownership/target/debug bin/ownership

clean-calling_c:
	-rm lib/libcalling_c.so

clean-calling_rust: src/calling_rust.c bin
	-rm bin/calling_rust

clean-memory_management: src/memory_management.c lib
	-rm lib/libmemory_management.so

clean-ownership: src/ownership.c bin
	-rm bin/ownership

clean-asynchronous_callbacks: src/asynchronous_callbacks.cpp lib
	-rm lib/libasynchronous_callbacks.so
