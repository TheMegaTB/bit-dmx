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


class Channel {
public:
    Channel();
    ChannelValue getValue(sf::Time time) const;
    ChannelValue getInterfaceValue() const;
    void setInterfaceValue(ChannelValue interfaceValue);
    
    void startFade(sf::Time startTime, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve, int uiElementID);
    void setValue(ChannelValue value, int uiElementID);
    
    void disableUIElement(int uiElementID, sf::Time now);
private:
    ChannelValue m_fadeStartValue;
    std::vector<ChannelValue> m_fadeEndValues;
    ChannelValue m_interfaceValue;
    
    std::vector<int> m_uiElementIDs;
    std::vector<sf::Time> m_fadeTimes;
    std::vector<FadeCurve> m_fadeCurves;
    
    sf::Time m_fadeStartTime;
    
    void removeUIElement(int elementID);
};

#endif /* Channel_hpp */
