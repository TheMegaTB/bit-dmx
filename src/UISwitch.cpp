//
//  Switch.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UISwitch.hpp"

UISwitch::UISwitch(Stage* stage, std::string caption, std::vector<int> channelGroups, std::vector<ChannelValue> channelValues): UIControlElement(stage, stage->UIPartWidth, stage->UIPartWidth / 4) {
    
    m_channels = channelGroups;
    m_channelValues = channelValues;
    
    
    m_toggle = std::make_shared<Toggle>([this](bool isActivated) -> void {
        if (isActivated) {
            this->activate();
        } else {
            this->deactivate();
        }
    }, caption, stage->UIPartWidth, stage->UIPartWidth / 4, m_stage->getFont());
                                        
    addPart(m_toggle);
}

void UISwitch::setCaption(std::string caption) {
    m_toggle->setCaption(caption);
}

void UISwitch::chaserActivate() {
    activate();
    m_toggle->setActivation(true);
}

void UISwitch::chaserDeactivate() {
    deactivate();
    m_toggle->setActivation(false);
}

void UISwitch::onHotkey() {
    UIControlElement::onHotkey();
    m_toggle->setActivation(m_isActivated);
}

void UISwitch::action() {
    for (int i = 0; i < m_channels.size(); i++) {
        m_stage->startFade(m_channels[i], m_fadeTime, m_channelValues[i], m_fadeCurve, m_id);
    }
}
