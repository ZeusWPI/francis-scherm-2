CC=gcc
CFLAGS=-O3 -Wall -Werror -Wpedantic
TARGET=francis-scherm-1

.PHONY: clean

build:
	${CC} ${CFLAGS} main.c -o ${TARGET} -lpthread

run:
	@./${TARGET}

clean:
	rm ${TARGET}
