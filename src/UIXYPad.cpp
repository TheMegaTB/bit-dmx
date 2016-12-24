//
//  UIXYPad.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIXYPad.hpp"

UIXYPad::UIXYPad(Stage* stage, UnvaluedActionGroup actionGroupX, UnvaluedActionGroup actionGroupY): UIControlElement(stage, stage->UIElementWidth, stage->UIElementWidth), m_virtualChannelX(-1), m_virtualChannelY(-1), m_actionGroupX(actionGroupX), m_actionGroupY(actionGroupY) {
    
    m_xyPad = std::make_shared<XYPad>(0, 255, stage->UIElementWidth, stage->UIElementWidth, m_stage->getFont());
    
    m_xyPad->onChange([this](double x, double y) -> void {
        m_virtualChannelX.setValue(x, SELF_ACTIVATION);
        m_virtualChannelY.setValue(y, SELF_ACTIVATION);
    });
    m_xyPad->onDisable([this]() -> void {
        m_virtualChannelX.deactivateActivation(m_stage->getNow(), SELF_ACTIVATION);
        m_virtualChannelY.deactivateActivation(m_stage->getNow(), SELF_ACTIVATION);
    });
    
    addElement(m_xyPad);
}

void UIXYPad::setValue(std::string subname, ChannelValue value, int activationID) {
    if (subname == "x") {
        m_virtualChannelX.setValue(value, activationID);
    } else if (subname == "y") {
        m_virtualChannelY.setValue(value, activationID);
    } else {
        std::cout << "There is no virtual channel named " << subname << " in a UIXYPad!" << std::endl;
    }
}

void UIXYPad::startFade(std::string subname, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve, int activationID) {
    if (subname == "x") {
       m_virtualChannelX.startFade(m_stage->getNow(), fadeTime, value, fadeCurve, activationID);
    } else if (subname == "y") {
        m_virtualChannelY.startFade(m_stage->getNow(), fadeTime, value, fadeCurve, activationID);
    } else {
        std::cout << "There is no virtual channel named " << subname << " in a UIXYPad!" << std::endl;
    }
}

void UIXYPad::deactivateActivation(int activationID) {
    m_virtualChannelX.deactivateActivation(m_stage->getNow(), activationID);
    m_virtualChannelY.deactivateActivation(m_stage->getNow(), activationID);
}

void UIXYPad::update() {
    
    bool updateChannelX = m_virtualChannelX.update(m_stage->getNow());
    bool updateChannelY = m_virtualChannelY.update(m_stage->getNow());
    
    if (updateChannelX) {
        ChannelValue x = m_virtualChannelX.getValue();
        if (x!= -1) {
            m_xyPad->setXValue(x);
            m_stage->activateActivationGroup(m_actionGroupX, x);
        } else {
            m_xyPad->setXValue(0);
            m_stage->deactivateActivationGroup(m_actionGroupX);
        }
    }
    
    if (updateChannelY) {
        ChannelValue y = m_virtualChannelY.getValue();
        if (y!= -1) {
            m_xyPad->setYValue(y);
            m_stage->activateActivationGroup(m_actionGroupY, y);
        } else {
            m_xyPad->setYValue(0);
            m_stage->deactivateActivationGroup(m_actionGroupY);
        }
    }
}
