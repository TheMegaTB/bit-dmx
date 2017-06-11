//
//  Switch.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UISwitch.hpp"

UISwitch::UISwitch(Stage* stage, ValuedActionGroup actionGroup) :m_actionGroup(actionGroup), UISingleVChannel(stage, stage->UIElementWidth, stage->UIElementWidth / 4) {
    
    m_toggle = std::make_shared<Toggle>("Untitled", stage->UIElementWidth, stage->UIElementWidth / 4, m_stage->getFont());
    
    m_toggle->onChange([this](bool isActivated) -> void {
        if (isActivated) {
            this->setValue("value", 255, SELF_ACTIVATION);
        } else {
            this->deactivateActivation(SELF_ACTIVATION);
        }
    });
                                        
    addElement(m_toggle);
}

void UISwitch::setCaption(std::string caption) {
    m_toggle->setCaption(caption);
}

void UISwitch::update() {
    if (m_virtualChannel.update(m_stage->getNow())) {
        if (isActivated()) {
            m_toggle->setActivation(true);
            m_stage->activateActivationGroup(m_actionGroup);
        } else {
            m_toggle->setActivation(false);
            m_stage->deactivateActivationGroup(m_actionGroup);
        }
    }
}

