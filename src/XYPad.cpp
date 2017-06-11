//
//  XYPad.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "XYPad.hpp"

XYPad::XYPad(int minValue, int maxValue, int width, int height, sf::Font font): Element(width, height) {
    m_minValue = minValue;
    m_maxValue = maxValue;
    m_xValue = 0;
    m_yValue = 1;
    m_font = font;
}

void XYPad::setRawValue(double xValue, double yValue) {
    if (m_xValue != xValue || m_yValue != yValue) {
        m_xValue = xValue;
        m_yValue = yValue;
        if (m_changeCallback) m_changeCallback(getXValue(), getYValue());
    }
}

int XYPad::getXValue() const {
    return round(m_minValue + (m_maxValue - m_minValue) * m_xValue);
}

int XYPad::getYValue() const {
    return round(m_minValue + (m_maxValue - m_minValue) * (1.f-m_yValue));
}

void XYPad::setValue(double xValue, double yValue) {
    setXValue(xValue);
    setYValue(yValue);
}

void XYPad::setXValue(double xValue) {
    m_xValue = (xValue - m_minValue)/(m_maxValue - m_minValue);
}

void XYPad::setYValue(double yValue) {
    m_yValue = 1 - (yValue - m_minValue)/(m_maxValue - m_minValue);
}

void XYPad::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {
    if (mouseButton == sf::Mouse::Left) {
        setRawValue((double)x / (double)getWidth(), (double)y / (double)getHeight());
    } else if (mouseButton == sf::Mouse::Right) {
        setRawValue(0, false);
        if (m_disableCallback) m_disableCallback();
    }
}

void XYPad::onMouseDrag(int x, int y, sf::Mouse::Button mouseButton) {
    if (mouseButton == sf::Mouse::Left) {
        setRawValue(fmin(fmax((double)x / (double)getWidth(), 0.f), 1.f), fmin(fmax((double)y / (double)getHeight(), 0.f), 1.f));
    }
}

void XYPad::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    states.transform *= getTransform();

    sf::RectangleShape background (sf::Vector2f(getWidth(), getHeight()));
    
    background.setFillColor(sf::Color::Yellow);
    background.setPosition(0, 0);

    target.draw(background, states);
    
    sf::VertexArray xLine(sf::Lines, 2);
    xLine[0].position = sf::Vector2f(m_xValue * getWidth(), 0);
    xLine[0].color  = sf::Color::Black;
    xLine[1].position = sf::Vector2f(m_xValue * getWidth(), getHeight());
    xLine[1].color = sf::Color::Black;
    target.draw(xLine, states);
    
    sf::VertexArray yLine(sf::Lines, 2);
    yLine[0].position = sf::Vector2f(0, m_yValue * getHeight());
    yLine[0].color  = sf::Color::Black;
    yLine[1].position = sf::Vector2f(getWidth(), m_yValue * getHeight());
    yLine[1].color = sf::Color::Black;
    target.draw(yLine, states);
    
    sf::Text caption = sf::Text("(" + std::to_string((int)getXValue()) + "|" + std::to_string((int)getYValue()) + ")", m_font, 12);
    caption.setPosition(0, 0);
    caption.setFillColor(sf::Color::Black);
    
    target.draw(caption, states);
    
}
