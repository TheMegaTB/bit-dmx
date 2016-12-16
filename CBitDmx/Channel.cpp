//
//  Channel.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Channel.hpp"

ChannelValue Channel::getValue(sf::Time time) const {
    if (time > m_fadeStartTime + m_fadeTime) {
        return m_fadeEndValue;
    } else {
        ChannelValue deltaValue = m_fadeEndValue - m_fadeStartValue;
        return m_fadeStartValue + calculateFadeCurve(m_fadeCurve, m_fadeTime/(time-m_fadeStartTime)) * deltaValue;
    }
}

void Channel::startFade(sf::Time startTime, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve) {
    m_fadeStartValue = getValue(startTime);
    m_fadeEndValue = value;
    m_fadeStartTime = startTime;
    m_fadeTime = fadeTime;
}

void Channel::setValue(ChannelValue value) {
    startFade(sf::Time::Zero, sf::Time::Zero, value, FadeCurve::linear);
}
