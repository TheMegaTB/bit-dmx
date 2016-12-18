//
//  ChannelGroup.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "ChannelGroup.hpp"


ChannelValue getFromVector(std::vector<ChannelValue> vector, int index) {
    if (index < vector.size()) {
        return vector[index];
    } else {
        return 0;
    }
}

ChannelGroup::ChannelGroup(Stage *stage, std::string name, ChannelGroupType channelGroupType, std::vector<int> channels) {
    m_stage = stage;
    m_name = name;
    m_channelGroupType = channelGroupType;
    m_channels = channels;
}

void ChannelGroup::startFade(sf::Time fadeTime, std::vector<ChannelValue> values, FadeCurve fadeCurve, int uiElementID) {
    for (int i = 0; i < m_channels.size(); i++) {
        m_stage->startFade(m_channels[i], fadeTime, getFromVector(values, i), fadeCurve, uiElementID);
    }
}

void ChannelGroup::setValue(std::vector<ChannelValue> values, int uiElementID) {
    for (int i = 0; i < m_channels.size(); i++) {
        m_stage->setValue(m_channels[i], getFromVector(values, i), uiElementID);
    }
}

int ChannelGroup::getChannelNumber() {
    return m_channels.size();
}

std::string ChannelGroup::getName() {
    return m_name;
}
