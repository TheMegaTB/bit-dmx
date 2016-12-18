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
    UISwitch(Stage* stage, std::string caption, std::vector<int> channelGroups);
    
    void setCaption(std::string caption);
    virtual void action();
    
    std::vector<std::vector<ChannelValue>> channelValues;
private:
    std::shared_ptr<Toggle> m_toggle;
    
    std::vector<int> m_channelGroups;
};


#endif /* UISwitch_hpp */
