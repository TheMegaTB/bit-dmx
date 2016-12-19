//
//  Toggle.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Toggle.hpp"

Toggle::Toggle(std::function<void(bool)> clickCallback, std::string caption, int width, int height, sf::Font font): UIPart(width, height) {
    m_clickCallback = clickCallback;
    m_caption = caption;
    m_font = font;
    m_activated = false;
}

void Toggle::setCaption(std::string caption) {
    m_caption = caption;
}

void Toggle::setActivation(bool activated) {
    m_activated = activated;
}

void Toggle::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {
    m_activated = !m_activated;
    m_clickCallback(m_activated);
}

void Toggle::draw(sf::RenderTarget& target, sf::RenderStates states) const {
    states.transform *= getTransform();
    
    sf::RectangleShape buttonShape (sf::Vector2f(getWidth(), getHeight()));
    if (m_activated) {
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
