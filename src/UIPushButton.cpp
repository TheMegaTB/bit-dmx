//
//  UIPushButton.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIPushButton.hpp"

UIPushButton::UIPushButton(Stage* stage, ValuedActionGroup actionGroup): m_actionGroup(actionGroup), UISingleVChannel(stage, stage->UIElementWidth, stage->UIElementWidth / 4) {
    
    m_button = std::make_shared<Button>("Untitled", stage->UIElementWidth, stage->UIElementWidth / 4, m_stage->getFont());
    
    m_button->onClick([this](bool isActivated) -> void {
        if (isActivated) {
            this->setValue("value", 255, SELF_ACTIVATION);
        } else {
            this->deactivateActivation(SELF_ACTIVATION);
        }
    });
    
    addElement(m_button);
}

void UIPushButton::setCaption(std::string caption) {
    m_button->setCaption(caption);
}

void UIPushButton::onHotkey() {
    if (!isActivated()) {
        setValue("value", 255, SELF_ACTIVATION);
    }
}

void UIPushButton::onHotkeyRelease() {
    deactivateActivation(SELF_ACTIVATION);
}

void UIPushButton::update() {
    if (m_virtualChannel.update(m_stage->getNow())) {
        if (isActivated()) {
            m_button->setPressed(true);
            m_stage->activateActivationGroup(m_actionGroup);
        } else {
            m_button->setPressed(false);
            m_stage->deactivateActivationGroup(m_actionGroup);
        }
    }
}
