//
//  Slider.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Slider.hpp"
#include <string>

Slider::Slider(int minValue, int maxValue, std::function<void(double)> valueChangeCallback, std::function<void()> disableCallback, int width, int height, sf::Font font): UIPart(width, height) {
    m_valueChangeCallback = valueChangeCallback;
    m_disableCallback = disableCallback;
    m_minValue = minValue;
    m_maxValue = maxValue;
    m_font = font;
}

void Slider::setRawValue(double value, bool callback) {
    if (m_value != value) {
        m_value = value;
        if (callback) {
            m_valueChangeCallback(getValue());
        }
    }
}

int Slider::getValue() const {
    return round(m_minValue + (m_maxValue - m_minValue) * m_value);
}

void Slider::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {
    if (mouseButton == sf::Mouse::Left) {
        setRawValue((double)x / (double)getWidth());
    } else if (mouseButton == sf::Mouse::Right) {
        setRawValue(0, false);
        m_disableCallback();
    }
}


void Slider::onMouseMove(int x, int y, sf::Mouse::Button mouseButton) {
    if (mouseButton == sf::Mouse::Left) {
        setRawValue(fmin(fmax((double)x / (double)getWidth(), 0.f), 1.f));
    }
}

void Slider::draw(sf::RenderTarget& target, sf::RenderStates states) const
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
