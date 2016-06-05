#include <DmxSimple.h>

unsigned int channel = 1;
unsigned int incomingByte;
unsigned char outgoingByte;
unsigned int maxChannel = 16;
bool newCycle = true;
bool mode = false; // true = channel, false = value
bool first_channel_byte = true;

void setup() {
  Serial.begin(115200);
  DmxSimple.usePin(11);
  DmxSimple.maxChannel(maxChannel);
  pinMode(13, OUTPUT);
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
          channel = incomingByte;
          first_channel_byte = false;
        } else if (mode) {
          channel = channel * 256 + incomingByte; //Combine the two bytes to one big integer
          if (channel > maxChannel) { maxChannel = channel; DmxSimple.maxChannel(channel); digitalWrite(13, HIGH); } //TODO: This may be causing issues if called
          newCycle = true;
          first_channel_byte = true;
        } else {
          DmxSimple.write(channel, incomingByte);
          newCycle = true;
        }
      }
      outgoingByte = ~incomingByte; //Negate the byte
      Serial.write(outgoingByte);
    }
  }
}
