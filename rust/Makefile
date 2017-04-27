.PHONY: build build-asynchronous_callbacks build-calling_c build-calling_rust build-memory_management build-ownership run run-asynchronous_callbacks run-calling_c run-calling_rust run-memory_management

run: run-asynchronous_callbacks run-calling_c run-memory_management
clean: clean-asynchronous_callbacks clean-calling_c clean-calling_rust clean-memory_management clean-ownership

build-asynchronous_callbacks:
	make -C asynchronous_callbacks build
build-calling_c:
	make -C calling_c build
build-calling_rust:
	make -C calling_rust build
build-memory_management:
	make -C memory_management build
build-ownership:
	make -C ownership build

run-asynchronous_callbacks:
	make -C asynchronous_callbacks run
run-calling_c:
	make -C calling_c run
run-memory_management:
	make -C memory_management run

clean-asynchronous_callbacks:
	make -C asynchronous_callbacks clean
clean-calling_c:
	make -C calling_c clean
clean-calling_rust:
	make -C calling_rust clean
clean-memory_management:
	make -C memory_management clean
clean-ownership:
	make -C ownership clean
