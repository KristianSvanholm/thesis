compile:
	g++ -c  -O3 -fomit-frame-pointer -march=native -fopenmp --std=c++14 mandelbrot.gpp-6.c++ -o mandelbrot.gpp-6.c++.o &&  g++ mandelbrot.gpp-6.c++.o -o mandelbrot.gpp-6.gpp_run -fopenmp 
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./mandelbrot.gpp-6.gpp_run 16000" C++ mandelbrot

run:
	./mandelbrot.gpp-6.gpp_run 16000

mem:
	/usr/bin/time -v ./mandelbrot.gpp-6.gpp_run 16000

valgrind:
	valgrind --tool=massif --stacks=yes ./mandelbrot.gpp-6.gpp_run 16000
