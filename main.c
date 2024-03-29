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
#include <unistd.h>

#define PACKET_SIZE 7

uint32_t line_length;
uint32_t bytes_per_pixel;
uint32_t frame_length;
volatile uint8_t *buffer;

void serve(uint16_t, uint16_t);

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

    serve((uint16_t) var_info.xres_virtual, (uint16_t) var_info.yres_virtual);

    return 0;
}

_Noreturn void serve(uint16_t x_res, uint16_t y_res) {
    printf("Starting server on port 8000\n");
    int listen_socket = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in server_addr = {
            .sin_family = AF_INET,
            .sin_port = htons(8000),
            .sin_addr.s_addr = inet_addr("0.0.0.0")
    };

    uint8_t size_bytes[] = {
        (x_res >> 0x8) & 0xff,
        x_res & 0xff,
        (y_res >> 0x8) & 0xff,
        y_res & 0xff,
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

        ssize_t written = write(client_socket, size_bytes, 4);
        if (written != 4) {
            continue;
        }

        pthread_t thread_id;
        pthread_create(&thread_id, NULL, &handle_socket, (void *) (uintptr_t) client_socket);
    }
}

void *handle_socket(void *socket) {
    int client_socket = (int) (uintptr_t) socket;

    uint8_t packet[1001*PACKET_SIZE] = {0};

    long res = 0;
    long bytes_left = 0;
    while ((res = recv(client_socket, &packet[bytes_left], PACKET_SIZE*1000, MSG_WAITFORONE)) != -1) {

        if (res == 0) {
            break;
        }

        res += bytes_left;
        bytes_left = res % PACKET_SIZE;

        for (int i = 0; i < (res / PACKET_SIZE); i++) {
            int base_i = i * PACKET_SIZE;
            unsigned int x = packet[base_i + 0] << 8 | packet[base_i + 1];
            unsigned int y = packet[base_i + 2] << 8 | packet[base_i + 3];
            unsigned int idx = y * line_length + x * bytes_per_pixel;


            if (idx >= frame_length) {
                continue;
            }

            *(buffer + idx + 0) = packet[base_i + 6];
            *(buffer + idx + 1) = packet[base_i + 5];
            *(buffer + idx + 2) = packet[base_i + 4];
        }

        bytes_left = res % PACKET_SIZE;
        if (bytes_left != 0) {
          for (int i = 0; i < bytes_left; i++) {
            packet[i] = packet[(res - (res % PACKET_SIZE)) + i];
          }
        }
    }

    return NULL;
}
