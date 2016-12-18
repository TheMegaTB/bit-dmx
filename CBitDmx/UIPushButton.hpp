//
//  UIPushButton.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIPushButton_hpp
#define UIPushButton_hpp

#include <stdio.h>

#include "UISingleHotkey.hpp"

class UIPushButton : public UISingleHotkey {
public:
    UIPushButton(Stage* stage, std::string caption, std::vector<int> channelGroups, std::vector<std::vector<ChannelValue>> channelValues, sf::Keyboard::Key hotkey);
    UIPushButton(Stage* stage, std::string caption, std::vector<int> channelGroups, sf::Keyboard::Key hotkey);
    UIPushButton(Stage* stage, json jsonObject) : UIPushButton(stage, jsonObject["caption"], jsonObject["channel_groups"], jsonObject["channel_values"], (sf::Keyboard::Key)jsonObject["hotkey"].get<int>()) {};
    
    virtual void onHotkey();
    virtual void onHotkeyRelease();
    
    void setCaption(std::string caption);
    virtual void action();
    
private:
    std::shared_ptr<Button> m_button;
    
    std::vector<int> m_channelGroups;
    std::vector<std::vector<ChannelValue>> m_channelValues;
};

#endif /* UIPushButton_hpp */
