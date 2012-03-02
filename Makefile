all: sotb

sotb: main.o
	gcc `sdl-config --libs` main.o -o sotb

main.o: main.c
	gcc -c `sdl-config --cflags` main.c

clean:
	rm -f main.o sotb
