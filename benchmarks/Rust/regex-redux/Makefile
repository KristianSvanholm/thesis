compile:
	cargo build --profile=release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./target/release/regex-redux 0 < ../../regexredux-input5000000.txt" Rust regex-redux

run:
	./target/release/regex-redux 0 < ../../regexredux-input5000000.txt

mem:
	/usr/bin/time -v ./target/release/regex-redux 0 < ../../regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./target/release/regex-redux 0 < ../../regexredux-input5000000.txt
