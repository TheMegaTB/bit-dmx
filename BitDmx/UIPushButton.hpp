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

#include "UIControlElement.hpp"

class UIPushButton : public UIControlElement {
public:
    UIPushButton(Stage* stage, std::string caption, std::vector<int> channels, std::vector<ChannelValue> channelValues);
    UIPushButton(Stage* stage, std::string caption, std::vector<int> channels) : UIPushButton(stage, caption, channels, std::vector<ChannelValue>(m_channels.size())) {};
    UIPushButton(Stage* stage, json jsonObject) : UIPushButton(stage, "Untitled", stage->getChannels(jsonObject["channels"]), jsonObject["channel_values"]) {};
    
    
    virtual void chaserActivate();
    virtual void chaserDeactivate();
    virtual void onHotkey();
    virtual void onHotkeyRelease();
    
    void setCaption(std::string caption);
    virtual void action();
    
private:
    std::shared_ptr<Button> m_button;
    
    std::vector<int> m_channels;
    std::vector<ChannelValue> m_channelValues;
};

#endif /* UIPushButton_hpp */
