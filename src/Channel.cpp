//
//  Channel.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Channel.hpp"

Channel::Channel(ChannelValue defaultValue) {
    m_fadeStartValue = defaultValue;
    m_fadeEndValues = {defaultValue};
    m_lastValue = 0;
    
    m_fadeStartTime = sf::Time::Zero;
    m_activations = {BASE_ACTIVATION};
    m_fadeTimes = {sf::Time::Zero};
    m_fadeCurves = {FadeCurve::linear};
}

bool Channel::update(sf::Time time) {
    ChannelValue new_value = m_fadeEndValues.back();
    
    if (!((m_fadeTimes.back().asMicroseconds() == 0) || (time > m_fadeStartTime + m_fadeTimes.back()))) {
        ChannelValue deltaValue = m_fadeEndValues.back() - m_fadeStartValue;
        new_value = m_fadeStartValue + calculateFadeCurve(m_fadeCurves.back(), (time-m_fadeStartTime) / m_fadeTimes.back()) * deltaValue;
    }
    
    if (new_value != m_lastValue) {
        m_lastValue = new_value;
        return true;
    } else return false;
}

void Channel::startFade(sf::Time startTime, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve, int activationID) {
    if (value == -1) {
        deactivateAll();
        return;
    }
    deactivateActivation(startTime, activationID);
    m_fadeStartValue = getValue();
    m_activations.push_back(activationID);
    m_fadeStartTime = startTime;
    m_fadeEndValues.push_back(value);
    m_fadeTimes.push_back(fadeTime);
    m_fadeCurves.push_back(fadeCurve);
}

void Channel::setValue(ChannelValue value, int activationID) {
    startFade(sf::Time::Zero, sf::Time::Zero, value, FadeCurve::linear, activationID);
}

void Channel::deactivateActivation(sf::Time now, int activationID) {
    for (int i = 0; i < m_activations.size(); i++) {
        if (m_activations[i] == activationID) {
            
            if (i == m_activations.size() - 1) {
                m_fadeStartValue = getValue();
                m_fadeStartTime = now;
            }
            m_activations.erase(m_activations.begin() + i);
            m_fadeTimes.erase(m_fadeTimes.begin() + i);
            m_fadeCurves.erase(m_fadeCurves.begin() + i);
            m_fadeEndValues.erase(m_fadeEndValues.begin() + i);
            return;
        }
    }
}

void Channel::deactivateAll() {
    m_fadeEndValues.resize(1);
    m_fadeStartValue = m_fadeEndValues[0];
    m_fadeStartTime = sf::Time::Zero;
    m_activations = {BASE_ACTIVATION};
    m_fadeTimes = {sf::Time::Zero};
    m_fadeCurves = {FadeCurve::linear};
}
