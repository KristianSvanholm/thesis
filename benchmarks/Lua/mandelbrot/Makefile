compile:

measure:
	sudo modprobe msr
	sudo ../../RAPL/main "lua  mandelbrot.lua-6.lua 12000" Lua mandelbrot

run:
	lua  mandelbrot.lua-6.lua 12000

mem:
	/usr/bin/time -v  lua  mandelbrot.lua-6.lua 12000

valgrind:
	valgrind --tool=massif --stacks=yes  lua  mandelbrot.lua-6.lua 12000
