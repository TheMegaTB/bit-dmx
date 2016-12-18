//
//  UIXYPad.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIXYPad_hpp
#define UIXYPad_hpp

#include <stdio.h>

#include "UILabeledElement.hpp"

class UIXYPad : public UILabeledElement {
public:
    UIXYPad(Stage* stage, ChannelAddress channelXAddress, ChannelAddress channelYAddress);
    UIXYPad(Stage* stage, json jsonObject) : UIXYPad(stage, stage->getChannel(jsonObject["channelx_address"]), stage->getChannel(jsonObject["channely_address"])) {};
    
    
    
    void setChannelAddress(ChannelAddress channelXAddress, ChannelAddress channelYAddress);
    virtual void action();
private:
    std::shared_ptr<XYPad> m_xyPad;
    
    ChannelAddress m_channelXAddress;
    ChannelAddress m_channelYAddress;
};

#endif /* UIXYPad_hpp */
