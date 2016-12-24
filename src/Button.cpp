//
//  Button.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Button.hpp"


Button::Button(std::string caption, int width, int height, sf::Font font): Element(width, height) {
    setCaption(caption);
    m_font = font;
    m_pressed = false;
    m_colorActivated = sf::Color(0xBF, 0xBF, 0xBF, 0xFF);
    m_colorDeactivated = sf::Color(0x80, 0x80, 0x80, 0xFF);
}

void Button::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {
    m_pressed = true;
    if (m_clickCallback) m_clickCallback(true);
}

void Button::onMouseRelease(int x, int y, sf::Mouse::Button mouseButton) {
    m_pressed = false;
    if (m_clickCallback) m_clickCallback(false);
}

void Button::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    states.transform *= getTransform();
    
    sf::RectangleShape buttonShape (sf::Vector2f(getWidth(), getHeight()));
    if (drawActivated()) {
        buttonShape.setFillColor(m_colorActivated);
    } else {
        buttonShape.setFillColor(m_colorDeactivated);
    }
    
    buttonShape.setOutlineThickness(1);
    buttonShape.setOutlineColor(sf::Color::Black);
    
    target.draw(buttonShape, states);
    
    sf::Text caption = sf::Text(m_caption, m_font, 12);
    caption.setPosition((getWidth() - caption.getLocalBounds().width) / 2, (getHeight() - caption.getLocalBounds().height) / 2);
    caption.setFillColor(sf::Color::Black);
    
    target.draw(caption, states);
}
