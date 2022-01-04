all: main

OBJ = main.o header1.o
HDR = header1.h
FLAG = -g

main: ${OBJ}
	g++ -o main ${OBJ} ${FLAG}


%.o: %.cpp ${HDR}
	g++ ${FLAG} -c $<

test: main
	./main

clean:
	rm -rf *.o main
