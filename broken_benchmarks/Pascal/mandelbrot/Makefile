compile:
	fpc -FuInclude/fpascal -XXs -O4 -Tlinux  -oFPASCAL_RUN mandelbrot.fpascal-5.pas
	mv FPASCAL_RUN mandelbrot.fpascal-5.fpascal_run

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./mandelbrot.fpascal-5.fpascal_run 12000" Pascal mandelbrot

run:
	./mandelbrot.fpascal-5.fpascal_run 12000

mem:
	/usr/bin/time -v  ./mandelbrot.fpascal-5.fpascal_run 12000

valgrind:
	valgrind --tool=massif --stacks=yes  ./mandelbrot.fpascal-5.fpascal_run 12000
