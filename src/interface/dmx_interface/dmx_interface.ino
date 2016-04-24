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

unsigned int channel = 0;
unsigned int incomingByte;
unsigned char outgoingByte;
unsigned int maxChannel = 32;
bool newCycle = true;
bool mode = false; // true = channel, false = value
bool first_channel_byte = true;

void setup() {
  Serial.begin(115200);
  DmxSimple.usePin(11);
  DmxSimple.maxChannel(maxChannel);
  pinMode(13, OUTPUT);
}

void endCycle() {
  newCycle = true;
  first_channel_byte = false;
}

void loop() {
  if (Serial.available() > 0) {
    while (Serial.available()) {
      incomingByte = Serial.read();
      if (newCycle && incomingByte == 0) { //value
        mode = false;
        newCycle = false;
      } else if (newCycle && incomingByte == 1) { //channel
        mode = true;
        newCycle = false;
      } else {
        if (mode && first_channel_byte) {
          channel = 0;
          channel = incomingByte;
          first_channel_byte = false;
        } else if (mode) {
          channel = channel * 256 + incomingByte; //Combine the two bytes to one big integer
          //if (channel > maxChannel) { maxChannel = channel; DmxSimple.maxChannel(channel); digitalWrite(13, HIGH); } //TODO: This may be causing issues if called
          //if (channel == 1) { digitalWrite(13, HIGH); } else { digitalWrite(13, LOW); }
          endCycle();
        } else {
          DmxSimple.write(channel, incomingByte);
          endCycle();
        }
      }
      outgoingByte = ~incomingByte; //Negate the byte
      Serial.write(outgoingByte);
    }
  }
}
