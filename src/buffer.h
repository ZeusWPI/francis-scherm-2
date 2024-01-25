#ifndef FS2_BUFFER
#define FS2_BUFFER

#include <stdint.h>

typedef struct buffer {
	uint16_t width;
	uint16_t height;
	volatile uint8_t* data;
	unsigned int size;
	uint8_t bytes_per_pixel;
} buffer_t;

buffer_t* buffer_init(uint16_t width, uint16_t height, uint8_t bytes_per_pixel);

void buffer_set(buffer_t* buffer, uint16_t x, uint16_t y, uint8_t r, uint8_t g, uint8_t b, uint8_t a);

#endif /* FS2_BUFFER */
