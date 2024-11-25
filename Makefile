all: final

final: main.o list.o
	gcc main.o list.o -o final

main.o: main.c
	gcc -c main.c

list.o: list.c
	gcc -c list.c

clean: rm main.o list.o final
