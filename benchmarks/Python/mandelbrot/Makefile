compile:
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "python -OO mandelbrot.python3-7.py 12000" Python mandelbrot

run:
	python -OO mandelbrot.python3-7.py 12000

mem:
	/usr/bin/time -v python -OO mandelbrot.python3-7.py 12000
