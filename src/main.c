#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

#include "buffer.h"
#include "display.h"
#include "server.h"

typedef struct server_thread_args {
	server_t* server;
	buffer_t* buffer;
} server_thread_args_t;

void* server_thread(void* args);

int main()
{
	display_t* display = display_init();
	server_t* server = server_init();
	buffer_t* buffer = buffer_init(display->width, display->height, display->bytes_per_pixel);

	server_thread_args_t s_args = {
		.server = server,
		.buffer = buffer
	};

	pthread_t s_thread;
	pthread_create(&s_thread, NULL, &server_thread, (void *) &s_args);

	while (1) {
		usleep(10 * 1000);
		display_render_frame(display, buffer);
		printf("Rendering Frame ...  \n");
	}

	return 0;
}

void* server_thread(void* args)
{
	server_thread_args_t* s_args = (server_thread_args_t*) args;

	server_run(s_args->server, s_args->buffer);

	return NULL;
}
