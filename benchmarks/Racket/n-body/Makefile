compile:
	raco make nbody.racket-2.racket

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "racket ./compiled/nbody.racket-2_racket.zo 50000000" Racket n-body

run:
	racket ./compiled/nbody.racket-2_racket.zo 50000000

mem:
	/usr/bin/time -v  racket ./compiled/nbody.racket-2_racket.zo 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  racket ./compiled/nbody.racket-2_racket.zo 50000000
