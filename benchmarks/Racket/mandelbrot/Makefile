compile:
	raco make mandelbrot.racket-3.racket

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "racket ./compiled/mandelbrot.racket-3_racket.zo 12000" Racket mandelbrot

run:
	racket ./compiled/mandelbrot.racket-3_racket.zo 12000

mem:
	/usr/bin/time -v racket ./compiled/mandelbrot.racket-3_racket.zo 12000

valgrind:
	valgrind --tool=massif --stacks=yes racket ./compiled/mandelbrot.racket-3_racket.zo 12000
