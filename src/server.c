#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>
#include <byteswap.h>

/* Socket Stuff */
#include <arpa/inet.h>
//#include <sys/socket.h>
#include <unistd.h>

#include "server.h"

#define PACKET_SIZE 7

typedef struct server_worker_thread_args {
	server_t* server;
	buffer_t* buffer;
	int socket;
} server_worker_thread_args_t;

void* server_worker_thread(void *);

server_t* server_init()
{
	printf("Initializing server\n");

	server_t* server = malloc(sizeof(server_t));

	if (server == NULL) {
		exit(EXIT_FAILURE);
	}

	struct sockaddr_in addr = {
		.sin_family = AF_INET,
		.sin_port = htons(8000),
		.sin_addr.s_addr = inet_addr("0.0.0.0")
	};

	server->socket = socket(AF_INET, SOCK_STREAM, 0);

	int error = bind(server->socket, (struct sockaddr*) &addr, sizeof(addr));

	if (error) {
		exit(EXIT_FAILURE);
	}

	return server;
}

_Noreturn void server_run(server_t* server, buffer_t* buffer)
{
	printf("Starting TCP server ... \n");
	listen(server->socket, 1);

	while (1) {
		struct sockaddr_in client_addr;
		int client_addr_len = sizeof(client_addr);
		int client_socket = accept(server->socket, (struct sockaddr *) &client_addr, (socklen_t *) &client_addr_len);

		uint16_t dimensions[2] = {
			buffer->width,
			buffer->height
		};

		ssize_t written = write(client_socket, dimensions, 4);
		if (written != 4) {
			continue;
		}

		server_worker_thread_args_t* s_args = malloc(sizeof(server_worker_thread_args_t));

		if (s_args == NULL) {
			printf("Error allocating arguments, skipping\n");
			continue;
		}

		s_args->server = server;
		s_args->buffer = buffer;
		s_args->socket = client_socket;

		printf("Accepted Connection\n");

		pthread_t thread_id;
		pthread_create(&thread_id, NULL, &server_worker_thread, (void *) s_args);
	}
}

void * server_worker_thread(void* args)
{
	server_worker_thread_args_t* s_args = (server_worker_thread_args_t*) args;

	long res = 0;
	uint8_t buffer[1000][PACKET_SIZE];

	while ((res = recv(s_args->socket, &buffer, 1000*PACKET_SIZE, MSG_WAITALL)) != -1) {
		if (res == 0) {
			break;
		}

		for (int i = 0; i < 1000; i++) {
			uint8_t *packet = buffer[i];

			uint16_t x = __bswap_constant_16(((uint16_t *) packet)[0]);
			uint16_t y = __bswap_constant_16(((uint16_t *) packet)[1]);

			buffer_set(s_args->buffer, x, y, packet[4], packet[5], packet[6], ~0);
		}
	}

	return NULL;
}