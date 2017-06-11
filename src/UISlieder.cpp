//
//  UIChannel.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UISlieder.hpp"

#include <iostream>

UISlieder::UISlieder(Stage* stage, UnvaluedActionGroup actionGroup): m_actionGroup(actionGroup), UISingleVChannel(stage, stage->UIElementWidth, stage->UIElementWidth / 4) {
    m_slider = std::make_shared<HorizontalSlider>(0, 255, stage->UIElementWidth, stage->UIElementWidth / 4, m_stage->getFont());
    
    m_slider->onChange([this](double x) -> void { this->setValue("value", x, SELF_ACTIVATION); });
    m_slider->onDisable([this]() -> void { this->deactivateActivation(SELF_ACTIVATION); });
    addElement(m_slider);
}

void UISlieder::update() {
    if (m_virtualChannel.update(m_stage->getNow())) {
        if (isActivated()) {
            ChannelValue value = m_virtualChannel.getValue();
            m_slider->setValue(value);
            m_stage->activateActivationGroup(m_actionGroup, value);
        } else {
            m_slider->setValue(0);
            m_stage->deactivateActivationGroup(m_actionGroup);
        }
    }
}
