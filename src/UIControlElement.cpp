//
//  UIElement.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIControlElement.hpp"
#include <iostream>


UIControlElement::UIControlElement(Stage* stage, int width, int height) : ElementController(width, height) {
    m_stage = stage;
    m_hotkey = sf::Keyboard::Unknown;
}

void UIControlElement::hotkeyWrapper(sf::Keyboard::Key hotkey) {
    if (m_hotkey == hotkey) {
        onHotkey();
    }
}

void UIControlElement::hotkeyReleaseWrapper(sf::Keyboard::Key hotkey) {
    if (m_hotkey == hotkey) {
        onHotkeyRelease();
    }
}
