//
//  Channel.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Channel.hpp"

Channel::Channel() {
    m_fadeStartValue = 0;
    m_fadeEndValues = {0}; //Default Value
    m_interfaceValue = 0;
    
    m_fadeStartTime = sf::Time::Zero;
    m_uiElementIDs = {-1};
    m_fadeTimes = {sf::Time::Zero};
    m_fadeCurves = {FadeCurve::linear};
}

ChannelValue Channel::getValue(sf::Time time) const {
    if ((m_fadeTimes.back().asMicroseconds() == 0) || (time > m_fadeStartTime + m_fadeTimes.back())) {
        return m_fadeEndValues.back();
    } else {
        ChannelValue deltaValue = m_fadeEndValues.back() - m_fadeStartValue;
        return m_fadeStartValue + calculateFadeCurve(m_fadeCurves.back(), (time-m_fadeStartTime) / m_fadeTimes.back()) * deltaValue;
    }
}

ChannelValue Channel::getInterfaceValue() const {
    return m_interfaceValue;
}

void Channel::setInterfaceValue(ChannelValue interfaceValue) {
    m_interfaceValue = interfaceValue;
}

void Channel::startFade(sf::Time startTime, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve, int uiElementID) {
    m_fadeStartValue = getValue(startTime);
    removeUIElement(uiElementID);
    m_uiElementIDs.push_back(uiElementID);
    m_fadeStartTime = startTime;
    m_fadeEndValues.push_back(value);
    m_fadeTimes.push_back(fadeTime);
    m_fadeCurves.push_back(fadeCurve);
}

void Channel::setValue(ChannelValue value, int uiElementID) {
    startFade(sf::Time::Zero, sf::Time::Zero, value, FadeCurve::linear, uiElementID);
}

void Channel::disableUIElement(int uiElementID, sf::Time now) {
    m_fadeStartValue = getValue(now);
    removeUIElement(uiElementID); //TODO check if its the last one -> activate the previous one
    m_fadeStartTime = now;
}

void Channel::removeUIElement(int elementID) {
    for (int i = 0; i < m_uiElementIDs.size(); i++) {
        if (m_uiElementIDs[i] == elementID) {
            m_uiElementIDs.erase(m_uiElementIDs.begin() + i);
            m_fadeTimes.erase(m_fadeTimes.begin() + i);
            m_fadeCurves.erase(m_fadeCurves.begin() + i);
            m_fadeEndValues.erase(m_fadeEndValues.begin() + i);
            return;
        }
    }
    
}
