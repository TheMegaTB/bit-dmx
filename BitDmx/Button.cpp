//
//  Button.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Button.hpp"


Button::Button(std::function<void(bool)> changeCallback, std::string caption, int width, int height, sf::Font font): UIPart(width, height) {
    m_changeCallback = changeCallback;
    setCaption(caption);
    m_font = font;
    m_pressed = false;
}

void Button::setPressed(bool pressed) {
    m_pressed = pressed;
}

void Button::setCaption(std::string caption) {
    m_caption = caption;
}

void Button::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {
    m_pressed = true;
    m_changeCallback(true);
}

void Button::onMouseRelease(int x, int y, sf::Mouse::Button mouseButton) {
    m_pressed = false;
    m_changeCallback(false);
}

void Button::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    states.transform *= getTransform();
    
    sf::RectangleShape buttonShape (sf::Vector2f(getWidth(), getHeight()));
    if (m_pressed) {
        buttonShape.setFillColor(sf::Color::Green);
    } else {
        buttonShape.setFillColor(sf::Color::Red);
    }
    
    buttonShape.setOutlineThickness(1);
    buttonShape.setOutlineColor(sf::Color::Black);
    
    target.draw(buttonShape, states);
    
    sf::Text caption = sf::Text(m_caption, m_font, 12);
    caption.setPosition((getWidth() - caption.getLocalBounds().width) / 2, (getHeight() - caption.getLocalBounds().height) / 2);
    caption.setFillColor(sf::Color::Black);
    
    target.draw(caption, states);
}
