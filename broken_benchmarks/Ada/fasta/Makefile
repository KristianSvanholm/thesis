compile:
	gnatchop -r -w fasta.gnat-5.gnat
	gnatmake -O3 -fomit-frame-pointer -march=native -msse3 -mfpmath=sse -gnatNp -f fasta.adb -o fasta.gnat-5.gnat_run 
	gnat compile -c -O3 -fomit-frame-pointer -march=native -msse3 -mfpmath=sse -gnatNp fasta.adb
	gnat compile -c -O3 -fomit-frame-pointer -march=native -msse3 -mfpmath=sse -gnatNp sequence.ads
	gnat compile -c -O3 -fomit-frame-pointer -march=native -msse3 -mfpmath=sse -gnatNp sequence-creation.adb
	gnat compile -c -O3 -fomit-frame-pointer -march=native -msse3 -mfpmath=sse -gnatNp sequence-data.ads
	gnat compile -c -O3 -fomit-frame-pointer -march=native -msse3 -mfpmath=sse -gnatNp lcg_random.adb
	gnat compile -c -O3 -fomit-frame-pointer -march=native -msse3 -mfpmath=sse -gnatNp line_io.adb
	gnatbind -x fasta.ali
	gnatlink fasta.ali -O3 -fomit-frame-pointer -march=native -msse3 -mfpmath=sse -o fasta.gnat-5.gnat_run

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "./fasta.gnat-5.gnat_run 25000000" Ada fasta

run:
	./fasta.gnat-5.gnat_run 25000000

mem:
	/usr/bin/time -v ./fasta.gnat-5.gnat_run 25000000

valgrind:
	valgrind --tool=massif --stacks=yes ./fasta.gnat-5.gnat_run 25000000
