//
//  ChannelGroup.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef ChannelGroup_hpp
#define ChannelGroup_hpp

class ChannelGroup;

#include <stdio.h>
#include "Stage.hpp"


class ChannelGroup {
public:
    ChannelGroup(Stage *stage, std::string name, std::vector<int> channels);
    void startFade(sf::Time fadeTime, std::vector<ChannelValue> value, FadeCurve fadeCurve, int uiElementID);
    void setValue(std::vector<ChannelValue> value, int uiElementID);
    int getChannelNumber();
private:
    Stage *m_stage;
    std::string m_name;
    std::vector<int> m_channels;
};

#endif /* ChannelGroup_hpp */
