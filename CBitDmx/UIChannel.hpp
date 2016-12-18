//
//  UIChannel.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIChannel_hpp
#define UIChannel_hpp

#include <stdio.h>

#include "UIControlElement.hpp"

class UIChannel : public UIControlElement {
public:
    UIChannel(Stage* stage, ChannelAddress channelAddress);
    UIChannel(Stage* stage, json jsonObject) : UIChannel(stage, stage->getChannel(jsonObject["channel_address"])) {};
    
    void setChannelAddress(ChannelAddress channelAddress);
    virtual void action();
private:
    std::shared_ptr<Slider> m_slider;
    
    ChannelAddress m_channelAddress;
};


#endif /* UIChannel_hpp */
