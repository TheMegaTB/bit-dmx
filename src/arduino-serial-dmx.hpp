//
//  arduino-serial-dmx.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/19/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef arduino_serial_dmx_h
#define arduino_serial_dmx_h

#include "arduino-serial-lib.hpp"

bool open_port(int baud, const char* port);
void write_dmx(uint16_t channel, uint8_t value);

#endif /* arduino_serial_dmx_h */
