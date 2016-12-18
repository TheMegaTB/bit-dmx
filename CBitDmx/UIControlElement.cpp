//
//  UIElement.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIControlElement.hpp"
#include <iostream>


UIControlElement::UIControlElement(Stage* stage, std::vector<std::shared_ptr<UIPart>> uiParts) : UIController(uiParts) {
    m_stage = stage;
    m_fadeTime = sf::seconds(1);
    m_fadeCurve = FadeCurve::linear;
    m_hotkey = sf::Keyboard::Unknown;
}

int UIControlElement::getHeight() const {
    return UIPartWidth / 4;
}

sf::Keyboard::Key UIControlElement::getHotkey() {
    return m_hotkey;
}

void UIControlElement::setHotkey(sf::Keyboard::Key hotkey) {
    m_hotkey = hotkey;
}

void UIControlElement::hotkeyWrapper(sf::Keyboard::Key hotkey) {
    if (hotkey == m_hotkey) {
        onHotkey();
    }
}

void UIControlElement::setID(int id) {
    m_id = id;
}

void UIControlElement::setFadeTime(sf::Time fadeTime) {
    m_fadeTime = fadeTime;
}

void UIControlElement::setFadeCurve(FadeCurve fadeCurve) {
    m_fadeCurve = fadeCurve;
}

void UIControlElement::activate() {
    m_isActivated = true;
    m_stage->activateUIElement(m_id);
}

void UIControlElement::deactivate() {
    m_isActivated = false;
    m_stage->deactivateUIElement(m_id);
}

void UIControlElement::action() {}

void UIControlElement::onHotkey() {
    if (m_isActivated) {
        deactivate();
    } else {
        activate();
    }
}
