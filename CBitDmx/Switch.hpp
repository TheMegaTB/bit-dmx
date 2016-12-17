//
//  Switch.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Switch_hpp
#define Switch_hpp

#include <stdio.h>
#include <iostream>

#include "UIElementActivatable.hpp"

class Switch : public UIElementActivateble {
public:
    Switch(Stage* stage, std::string name, std::vector<int> channelGroups);
    
    void setName(std::string name);
    
    virtual void activate();
    virtual void deactivate();
    virtual void action();
    
    std::vector<std::vector<ChannelValue>> channelValues;
private:
    std::string m_name;
    
    std::vector<int> m_channelGroups;
    sf::Text m_uiName;
    
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
};


#endif /* Switch_hpp */
