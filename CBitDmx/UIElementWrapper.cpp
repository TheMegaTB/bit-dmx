//
//  UIElementWrapper.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIElementWrapper.hpp"


UIElementWrapper::UIElementWrapper(std::shared_ptr<UIElement> ui_element) {
    uiElement = ui_element;
    m_hotkey = sf::Keyboard::Unknown;
}

sf::Keyboard::Key UIElementWrapper::getHotkey() {
    return m_hotkey;
}

void UIElementWrapper::setHotkey(sf::Keyboard::Key hotkey) {
    m_hotkey = hotkey;
}

void UIElementWrapper::onClick(int x, int y) {
    uiElement->onClick(x, y);
}

void UIElementWrapper::onHotkey(sf::Keyboard::Key hotkey) {
    if (hotkey == m_hotkey) {
        uiElement->onHotkey();
    }
}
