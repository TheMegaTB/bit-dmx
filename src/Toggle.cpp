//
//  Toggle.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Toggle.hpp"

Toggle::Toggle(std::string caption, int width, int height, sf::Font font): Button(caption, width, height, font) {
    m_activated = false;
}

void Toggle::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {
    Button::onMousePress(x, y, mouseButton);
    m_activated = !m_activated;
    if (m_changeCallback) m_changeCallback(m_activated);
}
