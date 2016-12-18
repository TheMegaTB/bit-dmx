//
//  UIPushButton.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIPushButton.hpp"

UIPushButton::UIPushButton(Stage* stage, std::string caption, std::vector<int> channels, std::vector<ChannelValue> channelValues, sf::Keyboard::Key hotkey): UISingleHotkey(stage, stage->UIPartWidth, stage->UIPartWidth / 4, hotkey) {
    m_channels = channels;
    
    m_channelValues = channelValues;
    
    m_button = std::make_shared<Button>([this](bool isActivated) -> void {
        if (isActivated) {
            this->activate();
        } else {
            this->deactivate();
        }
    }, caption, stage->UIPartWidth, stage->UIPartWidth / 4, m_stage->getFont());
    
    m_parts.push_back(m_button);
}

void UIPushButton::setCaption(std::string caption) {
    m_button->setCaption(caption);
}


void UIPushButton::onHotkey() {
    if (!m_isActivated) {
        activate();
        m_button->setPressed(true);
    }
}

void UIPushButton::onHotkeyRelease() {
    deactivate();
    m_button->setPressed(false);
}

void UIPushButton::action() {
    for (int i = 0; i < m_channels.size(); i++) {
        m_stage->startFade(m_channels[i], m_fadeTime, m_channelValues[i], m_fadeCurve, m_id);
    }
}
