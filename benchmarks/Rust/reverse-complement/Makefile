compile:
	cargo build --profile=release

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./target/release/reverse-complement 0 < ../../revcomp-input25000000.txt" Rust reverse-complement

run:
	./target/release/reverse-complement 0 < ../../revcomp-input25000000.txt

mem:
	/usr/bin/time -v ./target/release/reverse-complement 0 < ../../revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ./target/release/reverse-complement 0 < ../../revcomp-input25000000.txt
