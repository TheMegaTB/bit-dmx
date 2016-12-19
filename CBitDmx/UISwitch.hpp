//
//  Switch.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UISwitch_hpp
#define UISwitch_hpp

#include <stdio.h>

#include "UIControlElement.hpp"

class UISwitch : public UIControlElement {
public:
    UISwitch(Stage* stage, std::string caption, std::vector<int> channels, std::vector<ChannelValue> channelValues);
    UISwitch(Stage* stage, std::string caption, std::vector<int> channelGroups): UISwitch(stage, caption, channelGroups, std::vector<ChannelValue>(m_channels.size())) {};
    UISwitch(Stage* stage, json jsonObject) : UISwitch(stage, "Untitled", stage->getChannels(jsonObject["channels"]), jsonObject["channel_values"]) {};
    
    void setCaption(std::string caption);
    
    
    virtual void chaserActivate();
    virtual void chaserDeactivate();
    virtual void onHotkey();
    virtual void action();
    
    
    std::vector<ChannelValue> m_channelValues;
private:
    std::shared_ptr<Toggle> m_toggle;
    
    std::vector<int> m_channels;
};


#endif /* UISwitch_hpp */
