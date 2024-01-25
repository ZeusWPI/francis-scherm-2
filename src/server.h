#ifndef FS2_SERVER
#define FS2_SERVER

#include "buffer.h"

typedef struct server {
	int socket;
} server_t;

server_t* server_init();

_Noreturn void server_run(server_t*, buffer_t*);

#endif /* FS2_SERVER */
