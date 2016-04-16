/*
 * arduino-serial-dmx
 * ------------------
 *
 * A simple udp server that redirects everything it receives
 * to a connected serial device Works on any POSIX system (Mac/Unix/PC)
 *
 * Compile with something like:
 *   gcc -o arduino-serial-dmx arduino-serial-lib.c arduino-serial-dmx.c
 * or use the included Makefile
 *
 * Mac: make sure you have Xcode installed
 * Windows: try MinGW to get GCC
 *
 *
 * Originally created 5 December 2006
 * 2006-2013, Tod E. Kurt, http://todbot.com/blog/
 */

#include <stdio.h>    // Standard input/output definitions
#include <stdlib.h>
#include <string.h>   // String function definitions
#include <unistd.h>   // for usleep()
#include <sys/socket.h>
#include <netinet/in.h>

#include "arduino-serial-lib.h"

const char* port = "/dev/ttyACM0";
const int buf_max = 128;

int fd = -1;

void error(char* msg) {
    fprintf(stderr, "%s\n",msg);
    if( fd!=-1 ) {
        serialport_close(fd);
        printf("closed port\n");
    }
    exit(EXIT_FAILURE);
}

int open_port() {
    char serialport[buf_max];
    int baudrate = 230400;

    strcpy(serialport, port);
    fd = serialport_init(port, baudrate);
    if( fd==-1 ) { printf("couldn't open port"); } else {
        printf("opened port %s\n",serialport);
        serialport_flush(fd);
    }
    return fd;
}

void write_to_serial(char* msg) {
    int i;

    if( fd==-1 ) printf("port not opened.");
    char buf[strlen(msg)+1];
    sprintf(buf, "%s\n", msg);
    int rc = serialport_write(fd, buf);
    if(rc == -1) {
        printf("error writing");
        while(1) { open_port(); usleep( 1000 * 1000 ); } //retry connecting every second
        fd = -1;
    }
}

void udp_server() {
    int udpSocket, nBytes;
    char buffer[128];
    struct sockaddr_in serverAddr, clientAddr;
    struct sockaddr_storage serverStorage;
    socklen_t addr_size, client_addr_size;
    int i;

    /*Create UDP socket*/
    udpSocket = socket(PF_INET, SOCK_DGRAM, 0);

    /*Configure settings in address struct*/
    serverAddr.sin_family = AF_INET;
    serverAddr.sin_port = htons(7777);
    serverAddr.sin_addr.s_addr = inet_addr("127.0.0.1");
    memset(serverAddr.sin_zero, '\0', sizeof serverAddr.sin_zero);

    /*Bind socket with address struct*/
    bind(udpSocket, (struct sockaddr *) &serverAddr, sizeof(serverAddr));

    /*Initialize size variable to be used later on*/
    addr_size = sizeof serverStorage;

    while(1) {
        /* Try to receive any incoming UDP datagram. Address and port of
          requesting client will be stored on serverStorage variable */
        nBytes = recvfrom(udpSocket,buffer,128,0,(struct sockaddr *)&serverStorage, &addr_size);

        printf("%s\n", buffer);
        write_to_serial(buffer);
        memset(buffer, 0, sizeof buffer);
    }
}

int main(int argc, char *argv[]) {
    open_port();

    udp_server();

    exit(EXIT_SUCCESS);
}
