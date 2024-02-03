CC=gcc
CFLAGS=-O3 -Wall -Werror -Wpedantic
TARGET=francis-scherm-1

BUILD_DIR=build

.PHONY: clean

build-old:
	${CC} ${CFLAGS} main.c -o ${TARGET} -lpthread

run:
	@./${TARGET}

clean:
	rm ${TARGET}
	rm build/*

build: ${BUILD_DIR}/server.o ${BUILD_DIR}/display_sdl.o ${BUILD_DIR}/buffer.o
	${CC} ${CFLAGS} -o ${TARGET} \
		src/main.c \
		${BUILD_DIR}/display_sdl.o \
		${BUILD_DIR}/server.o \
		${BUILD_DIR}/buffer.o \
		-lpthread -lSDL2 -lSDL2_ttf

$(BUILD_DIR)/server.o: src/server.c src/server.h
	${CC} ${CFLAGS} src/server.c -o ${BUILD_DIR}/server.o -c

$(BUILD_DIR)/display_fb.o: src/display_fb.c src/display.h
	${CC} ${CFLAGS} src/display_fb.c -o ${BUILD_DIR}/display_fb.o -c

$(BUILD_DIR)/display_sdl.o: src/display_sdl.c src/display.h
	${CC} ${CFLAGS} src/display_sdl.c -o ${BUILD_DIR}/display_sdl.o -c

$(BUILD_DIR)/buffer.o: src/buffer.c src/buffer.h
	${CC} ${CFLAGS} src/buffer.c -o ${BUILD_DIR}/buffer.o -c
