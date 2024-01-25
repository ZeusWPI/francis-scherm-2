#include <stdlib.h>
#include <stdio.h>

#include "buffer.h"

#define BYTES_PER_PIXEL 4

buffer_t* buffer_init(uint16_t width, uint16_t height, uint8_t bytes_per_pixel)
{
	printf("Initializing buffer\n");

	buffer_t* buffer = malloc(sizeof(buffer_t));

	if (buffer == NULL) {
		exit(EXIT_FAILURE);
	}

	buffer->width = width;
	buffer->height = height;
	buffer->size = width * height * bytes_per_pixel;
	buffer->bytes_per_pixel = bytes_per_pixel;
	buffer->data = malloc(buffer->size);

	if (buffer->data == NULL) {
		exit(EXIT_FAILURE);
	}

	return buffer;
}

void buffer_set(buffer_t* buffer, uint16_t x, uint16_t y, uint8_t r, uint8_t g, uint8_t b, uint8_t a)
{
	unsigned int idx = y * buffer->width * buffer->bytes_per_pixel + x * buffer->bytes_per_pixel;

	if (idx >= buffer->size) {
		return;
	}

	*(buffer->data + idx + 0) = b;
	*(buffer->data + idx + 1) = g;
	*(buffer->data + idx + 2) = r;
	*(buffer->data + idx + 3) = a;
}
