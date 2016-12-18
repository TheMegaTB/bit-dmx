//
//  Switch.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UISwitch.hpp"

UISwitch::UISwitch(Stage* stage, std::string caption, std::vector<int> channelGroups): UIControlElement(stage, {}) {
    m_channelGroups = channelGroups;
    
    channelValues.resize(channelGroups.size());
    
    for (int i = 0; i < m_channelGroups.size(); i++) {
        channelValues[i].resize(m_stage->getChannelGroup(channelGroups[i])->getChannelNumber());
    }
    
    m_toggle = std::make_shared<Toggle>(Toggle([this](bool isActivated) -> void {
        if (isActivated) {
            this->activate();
        } else {
            this->deactivate();
        }
    }, caption, m_stage->getFont()));
                                        
    m_uiParts.push_back(m_toggle);
}

void UISwitch::setCaption(std::string caption) {
    m_toggle->setCaption(caption);
}

void UISwitch::action() {
    for (int i = 0; i < m_channelGroups.size(); i++) {
        m_stage->startFadeForChannelGroup(m_channelGroups[i], m_fadeTime, channelValues[i], m_fadeCurve, m_id);
    }
}
