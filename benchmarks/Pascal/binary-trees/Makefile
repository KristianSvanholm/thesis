compile:
	fpc -FuInclude/fpascal -XXs -O4 -Tlinux  -oFPASCAL_RUN binarytrees.pas
	mv FPASCAL_RUN binarytrees.fpascal_run
	
measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./binarytrees.fpascal_run 21" Pascal binary-trees

run:
	./binarytrees.fpascal_run 21

mem:
	/usr/bin/time -v ./binarytrees.fpascal_run 21

valgrind:
	valgrind --tool=massif --stacks=yes ./binarytrees.fpascal_run 21
