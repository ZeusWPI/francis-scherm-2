#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <stdint.h>


/* Screen buffer Stuff */
#include <linux/fb.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/mman.h>

/* Socket Stuff */
#include <arpa/inet.h>
#include <sys/socket.h>

#define PACKET_SIZE 7

uint32_t line_length;
uint32_t bytes_per_pixel;
uint32_t frame_length;
volatile uint8_t *buffer;

void serve();

void *handle_socket(void *);

int main(int argc, char **argv) {
    int fd_screen = open("/dev/fb0", O_RDWR);
    struct fb_var_screeninfo var_info;
    struct fb_fix_screeninfo fix_info;
    ioctl(fd_screen, FBIOGET_VSCREENINFO, &var_info);
    ioctl(fd_screen, FBIOGET_FSCREENINFO, &fix_info);

    line_length = fix_info.line_length;
    bytes_per_pixel = var_info.bits_per_pixel / 8;

    frame_length = line_length * var_info.yres_virtual;

    buffer = mmap(0, frame_length, PROT_READ | PROT_WRITE, MAP_SHARED, fd_screen, 0);

    serve();

    return 0;
}

_Noreturn void serve() {
    printf("Starting server on port 8000\n");
    int listen_socket = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in server_addr = {
            .sin_family = AF_INET,
            .sin_port = htons(8000),
            .sin_addr.s_addr = inet_addr("0.0.0.0")
    };

    int err = bind(listen_socket, (struct sockaddr *) &server_addr, sizeof(server_addr));
    if (err) {
        printf("Errno: %d\n", errno);
        printf("Error message: %s\n", strerror(errno));

        exit(EXIT_FAILURE);
    }

    listen(listen_socket, 1);

    while (1) {
        struct sockaddr_in client_addr;
        int client_addr_len = sizeof(client_addr);
        int client_socket = accept(listen_socket, (struct sockaddr *) &client_addr, (socklen_t *) &client_addr_len);


        pthread_t thread_id;
        pthread_create(&thread_id, NULL, &handle_socket, (void *) (uintptr_t) client_socket);
    }
}

void *handle_socket(void *socket) {
    int client_socket = (int) (uintptr_t) socket;

    uint8_t packet[1000][PACKET_SIZE] = {0};

    long res;
    while ((res = recv(client_socket, &packet, PACKET_SIZE*1000, MSG_WAITALL)) != -1) {

        if (res == 0) {
            break;
        }

        if (res != PACKET_SIZE*1000) {
            continue;
        }

        for (int i = 0; i < 1000; i++) {
            unsigned int x = packet[i][0] << 8 | packet[i][1];
            unsigned int y = packet[i][2] << 8 | packet[i][3];
            unsigned int idx = y * line_length + x * bytes_per_pixel;


            if (idx >= frame_length) {
                continue;
            }

            *(buffer + idx) = packet[i][6];
            *(buffer + idx + 1) = packet[i][5];
            *(buffer + idx + 2) = packet[i][4];
        }
    }

    return NULL;
}
