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

#include "UISingleHotkey.hpp"

class UISwitch : public UISingleHotkey {
public:
    UISwitch(Stage* stage, std::string caption, std::vector<int> channelGroups, std::vector<std::vector<ChannelValue>> channelValues, sf::Keyboard::Key hotkey);
    UISwitch(Stage* stage, std::string caption, std::vector<int> channelGroups, sf::Keyboard::Key hotkey);
    UISwitch(Stage* stage, json jsonObject) : UISwitch(stage, jsonObject["caption"], jsonObject["channel_groups"], jsonObject["channel_values"], (sf::Keyboard::Key)jsonObject["hotkey"].get<int>()) {};
    
    void setCaption(std::string caption);
    
    
    virtual void onHotkey();
    virtual void action();
    
    
    std::vector<std::vector<ChannelValue>> m_channelValues;
private:
    std::shared_ptr<Toggle> m_toggle;
    
    std::vector<int> m_channelGroups;
};


#endif /* UISwitch_hpp */
