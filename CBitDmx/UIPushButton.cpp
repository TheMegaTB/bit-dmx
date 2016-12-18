//
//  UIPushButton.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIPushButton.hpp"

UIPushButton::UIPushButton(Stage* stage, std::string caption, std::vector<int> channelGroups): UIControlElement(stage, {}) {
    m_channelGroups = channelGroups;
    
    channelValues.resize(channelGroups.size());
    
    for (int i = 0; i < m_channelGroups.size(); i++) {
        channelValues[i].resize(m_stage->getChannelGroup(channelGroups[i])->getChannelNumber());
    }
    
    m_button = std::make_shared<Button>(Button([this](bool isActivated) -> void {
        if (isActivated) {
            this->activate();
        } else {
            this->deactivate();
        }
    }, caption, m_stage->getFont()));
    
    m_uiParts.push_back(m_button);
}

void UIPushButton::setCaption(std::string caption) {
    m_button->setCaption(caption);
}

void UIPushButton::action() {
    for (int i = 0; i < m_channelGroups.size(); i++) {
        m_stage->startFadeForChannelGroup(m_channelGroups[i], m_fadeTime, channelValues[i], m_fadeCurve, m_id);
    }
}
