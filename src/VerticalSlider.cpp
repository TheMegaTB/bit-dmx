//
//  VerticalSlider.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "VerticalSlider.hpp"
#include <string>

VerticalSlider::VerticalSlider(int minValue, int maxValue, int width, int height, sf::Font font): Element(width, height) {
    m_minValue = minValue;
    m_maxValue = maxValue;
    m_font = font;
}

void VerticalSlider::setRawValue(double value) {
    if (m_value != value) {
        m_value = value;
        if (m_changeCallback) m_changeCallback(getValue());
    }
}

void VerticalSlider::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {
    if (mouseButton == sf::Mouse::Left) {
        setRawValue((double)x / (double)getWidth());
    } else if (mouseButton == sf::Mouse::Right) {
        setRawValue(0);
        if (m_disableCallback) m_disableCallback();
    }
}

void VerticalSlider::onMouseDrag(int x, int y, sf::Mouse::Button mouseButton) {
    if (mouseButton == sf::Mouse::Left) {
        setRawValue(fmin(fmax((double)y / (double)getHeight(), 0.f), 1.f));
    }
}

void VerticalSlider::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    states.transform *= getTransform();
    
    int activeWidth = m_value * getWidth();
    
    sf::RectangleShape activated (sf::Vector2f(activeWidth, getHeight()));
    sf::RectangleShape deactivated (sf::Vector2f(getWidth() - activeWidth, getHeight()));
    
    activated.setFillColor(sf::Color::Green);
    deactivated.setFillColor(sf::Color::Yellow);
    
    activated.setPosition(0, 0);
    deactivated.setPosition(activeWidth, 0);
    
    target.draw(activated, states);
    target.draw(deactivated, states);
    
    sf::Text caption = sf::Text("Value: " + std::to_string((int)round(getValue())), m_font, 12);
    caption.setPosition((getWidth() - caption.getLocalBounds().width) / 2, (getHeight() - caption.getLocalBounds().height) / 2);
    caption.setFillColor(sf::Color::Black);
    
    target.draw(caption, states);

}
