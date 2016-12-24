//
//  Channel.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Channel_hpp
#define Channel_hpp

#include <stdio.h>
#include <vector>

#include <SFML/System/Time.hpp>

#include "Types.hpp"
#include "FadeCurve.hpp"
#include <iostream>

class Channel {
public:
    Channel(ChannelValue defaultValue = 0);
    
    ChannelValue getValue() const { return m_lastValue; };
    bool update(sf::Time time);
    
    void setValue(ChannelValue value, int activationID);
    void startFade(sf::Time startTime, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve, int activationID);
    void deactivateActivation(sf::Time now, int activationID);
    
    void deactivateAll();
private:
    ChannelValue m_lastValue;
    
    //current fade
    ChannelValue m_fadeStartValue;
    sf::Time m_fadeStartTime;
    //fade history
    std::vector<int> m_activations;
    std::vector<ChannelValue> m_fadeEndValues;
    std::vector<sf::Time> m_fadeTimes;
    std::vector<FadeCurve> m_fadeCurves;
};

#endif /* Channel_hpp */
