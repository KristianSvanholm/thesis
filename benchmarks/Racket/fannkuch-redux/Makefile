compile:
	raco make fannkuchredux.racket-3.racket

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "racket ./compiled/fannkuchredux.racket-3_racket.zo 12" Racket fannkuch-redux

run:
	racket ./compiled/fannkuchredux.racket-3_racket.zo 12

mem:
	/usr/bin/time -v racket ./compiled/fannkuchredux.racket-3_racket.zo 12

valgrind:
	valgrind --tool=massif --stacks=yes racket ./compiled/fannkuchredux.racket-3_racket.zo 12
