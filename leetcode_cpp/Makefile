all: main

OBJ = main.o header1.o header2.o
HDR = header1.h
FLAG = -g -std=c++17 -fsanitize=address -stdlib=libstdc++
CC = clang++

main: ${OBJ}
	${CC} -o main ${OBJ} ${FLAG}


%.o: %.cpp ${HDR}
	${CC} ${FLAG} -c $<

test: main
	./main

clean:
	rm -rf *.o main
