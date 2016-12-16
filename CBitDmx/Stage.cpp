//
//  Stage.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Stage.hpp"

Stage::Stage(int universeSize) {
    m_channels.resize(universeSize);
}

bool Stage::setValue(ChannelAddress address, ChannelValue value) {
    if (address < m_channels.size()) {
        m_channels[address].setValue(value);
        return true;
    } else {
        return false;
    }
}

bool Stage::startFade(ChannelAddress address, sf::Time startTime, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve) {
    if (address < m_channels.size()) {
        m_channels[address].startFade(startTime, fadeTime, value, fadeCurve);
        return true;
    } else {
        return false;
    }
}

ChannelValue Stage::getValue(ChannelAddress address, sf::Time time) const {
    if (address < m_channels.size()) {
        return m_channels[address].getValue(time);
    } else {
        return -1;
    }
}

bool Stage::updateChannel(ChannelAddress address) {
    return true; //TODO implement
}
