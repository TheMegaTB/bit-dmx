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
    UIPushButton(Stage* stage, std::string caption, std::vector<int> channelGroups);
    
    void setCaption(std::string caption);
    virtual void action();
    
    std::vector<std::vector<ChannelValue>> channelValues;
private:
    std::shared_ptr<Button> m_button;
    
    std::vector<int> m_channelGroups;
};

#endif /* UIPushButton_hpp */
