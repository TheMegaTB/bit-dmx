/* This program allows you to set DMX channels over the serial port.
**
** After uploading to Arduino, switch to Serial Monitor and set the baud rate
** to 9600. You can then set DMX channels using these commands:
**
** <number>c : Select DMX channel
** <number>v : Set DMX channel to new value
**
** These can be combined. For example:
** 100c355w : Set channel 100 to value 255.
**
** For more details, and compatible Processing sketch,
** visit http://code.google.com/p/tinkerit/wiki/SerialToDmx
**
** Help and support: http://groups.google.com/group/dmxsimple       */

#include <DmxSimple.h>

void setup() {
  Serial.begin(115200);
//  Serial.println("SerialToDmx ready");
//  Serial.println();
//  Serial.println("Syntax:");
//  Serial.println(" 123c : use DMX channel 123");
//  Serial.println(" 45w  : set current channel to value 45");
  DmxSimple.usePin(11);
  DmxSimple.maxChannel(16);
  digitalWrite(12, HIGH);
}

unsigned int channel;
int incomingByte;
unsigned int maxChannel;
bool newCycle = true;
bool mode = false; // true = channel, false = value
bool first_channel_byte = false;

void endCycle() {
  newCycle = true;
  first_channel_byte = false;
}

void loop() {
  if (Serial.available() > 0) {
    incomingByte = Serial.read();
    if (newCycle && incomingByte == 0) {
      mode = false;
    } else if (newCycle && incomingByte == 1) {
      mode = true;
    } else {
      if (mode && first_channel_byte) {
        channel = incomingByte;
      } else if (mode) {
        channel = channel * 256 + incomingByte; //Combine the two bytes to one big integer
        if (channel > maxChannel) { maxChannel = channel; }
        endCycle();
      } else {
        DmxSimple.write(channel, incomingByte);
      }
    }
  }

//  int c;
//
//  while(!Serial.available());
//  c = Serial.read();
//  if ((c>='0') && (c<='9')) {
//    value = 10*value + c - '0';
//  } else {
//    if (c=='c') channel = value;
//    else if (c=='w') {
//      DmxSimple.write(channel, value);
//      Serial.println();
//    }
//    value = 0;
//  }
}
