//
//  Stage.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Stage_hpp
#define Stage_hpp

#include <stdio.h>
#include <vector>

#include "Channel.hpp"

class Stage {
public:
    Stage(int universeSize);
    bool setValue(ChannelAddress address, ChannelValue value);
    bool startFade(ChannelAddress address, sf::Time startTime, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve);
    
    ChannelValue getValue(ChannelAddress address, sf::Time time) const;
private:
    bool updateChannel(ChannelAddress address);
    
    std::vector<Channel> m_channels;
};

#endif /* Stage_hpp */
