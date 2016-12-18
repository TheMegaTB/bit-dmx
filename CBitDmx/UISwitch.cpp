//
//  Switch.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UISwitch.hpp"

UISwitch::UISwitch(Stage* stage, std::string caption, std::vector<int> channelGroups, std::vector<std::vector<ChannelValue>> channelValues, sf::Keyboard::Key hotkey): UISingleHotkey(stage, stage->UIPartWidth, stage->UIPartWidth / 4, hotkey) {
    m_channelGroups = channelGroups;
    
    m_channelValues = channelValues;
    
    
    m_toggle = std::make_shared<Toggle>([this](bool isActivated) -> void {
        if (isActivated) {
            this->activate();
        } else {
            this->deactivate();
        }
    }, caption, stage->UIPartWidth, stage->UIPartWidth / 4, m_stage->getFont());
                                        
    m_parts.push_back(m_toggle);
}

UISwitch::UISwitch(Stage* stage, std::string caption, std::vector<int> channelGroups, sf::Keyboard::Key hotkey) : UISwitch(stage, caption, channelGroups, std::vector<std::vector<ChannelValue>>(), hotkey) {
    m_channelValues.resize(m_channelGroups.size());
    
    for (int i = 0; i < m_channelGroups.size(); i++) {
        m_channelValues[i].resize(m_stage->getChannelGroup(channelGroups[i])->getChannelNumber());
    }
}

void UISwitch::setCaption(std::string caption) {
    m_toggle->setCaption(caption);
}


void UISwitch::onHotkey() {
    UISingleHotkey::onHotkey();
    m_toggle->setActivation(m_isActivated);
}

void UISwitch::action() {
    for (int i = 0; i < m_channelGroups.size(); i++) {
        m_stage->startFadeForChannelGroup(m_channelGroups[i], m_fadeTime, m_channelValues[i], m_fadeCurve, m_id);
    }
}
