compile:
	raco make revcomp.racket-2.racket

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "racket ./compiled/revcomp.racket-2_racket.zo 0 < ../../revcomp-input25000000.txt" Racket reverse-complement

run:
	racket ./compiled/revcomp.racket-2_racket.zo 0 < ../../revcomp-input25000000.txt

mem:
	/usr/bin/time -v racket ./compiled/revcomp.racket-2_racket.zo 0 < ../../revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes racket ./compiled/revcomp.racket-2_racket.zo 0 < ../../revcomp-input25000000.txt
