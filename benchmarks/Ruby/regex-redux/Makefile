compile:


measure:
	sudo modprobe msr
	sudo ../../RAPL/main "ruby -W0 regexredux.yarv-2.yarv 0 < ../../regexredux-input5000000.txt" Ruby regex-redux

run:
	ruby -W0 regexredux.yarv-2.yarv 0 < ../../regexredux-input5000000.txt

mem:
	/usr/bin/time -v ruby -W0 regexredux.yarv-2.yarv 0 < ../../regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ruby -W0 regexredux.yarv-2.yarv 0 < ../../regexredux-input5000000.txt
