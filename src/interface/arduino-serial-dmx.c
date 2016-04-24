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
#include <arpa/inet.h> // inet_addr
#include <sys/socket.h>
#include <netinet/in.h>
#include <stdbool.h>

#include "arduino-serial-lib.h"

const int buf_max = 10;

int fd = -1;
int baudrate;
char buf[10];
char port_addr[100];
bool connected = false;
uint16_t prev_channel;

bool open_port(int baud, char* port) {
    strcpy(port_addr, port);
    baudrate = baud;
    fd = serialport_init(port, baudrate);
    serialport_flush(fd);
    if (fd != -1) { connected = true; return true; } else { return false; }
    return fd;
}

void close_port() {
  serialport_flush(fd);
  serialport_close(fd);
  connected = false;
  fd = -1;
}

bool reconnect() {
  close_port();
  for (int i = 0; i < 10; i++) {
    if (open_port(115200, "/dev/ttyACM0")) { return true; }
  }
  return false;
}

bool is_connected() {
  if (fd == -1 || connected == false) { return false; } else { return true; }
}

int write_to_serial(uint8_t b) {
  memset(buf, 0, sizeof buf);
  int result = serialport_writebyte(fd, b);
  if (serialport_read(fd, buf, 1, 5000) < 0) { printf("READ FAILED\n"); }
  uint8_t response = ~buf[0];
  if (response != b) {
    connected = false;
    result = -1;
    printf("CHECKSUM MISMATCH\n");
    printf("%d\n", b);
    printf("%d\n", response);
  }

  return result; //-1 equals there was a write error
}

void write_dmx(uint16_t channel, uint8_t value) { //TODO: Write to file. Format: Byte 1 = Value of channel 1 ... Byte n = Value of channel n
  if (channel != prev_channel) {
    uint8_t clow = channel & 0xff;
    uint8_t chigh = (channel >> 8);
    write_to_serial(1); //get into channel mode
    write_to_serial(chigh);
    write_to_serial(clow);
    prev_channel = channel;
  }
  write_to_serial(0); //get into value mode
  write_to_serial(value);
  //TODO: Return value
}
