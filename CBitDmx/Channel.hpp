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
#include <SFML/System/Time.hpp>

#include "Types.hpp"
#include "FadeCurve.cpp"


class Channel {
public:
    ChannelValue getValue(sf::Time time) const;
    
    void startFade(sf::Time startTime, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve);
    void setValue(ChannelValue value);
private:
    ChannelAddress m_address;
    
    ChannelValue m_fadeStartValue;
    ChannelValue m_fadeEndValue;
    
    sf::Time m_fadeStartTime;
    sf::Time m_fadeTime;
    
    FadeCurve m_fadeCurve;
};

#endif /* Channel_hpp */
