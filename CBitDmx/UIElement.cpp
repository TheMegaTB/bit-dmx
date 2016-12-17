//
//  UIElement.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include <iostream>

#include "UIElement.hpp"


UIElement::UIElement(Stage * stage) {
    m_width = UIElementWidth;
    m_stage = stage;
}

int UIElement::getHeight() const {
    return m_width / 4;
}

void UIElement::setID(int id) {
    m_id = id;
}

void UIElement::setFadeTime(sf::Time fadeTime) {
    m_fadeTime = fadeTime;
}

void UIElement::setFadeCurve(FadeCurve fadeCurve) {
    m_fadeCurve = fadeCurve;
}

void UIElement::action() {}

void UIElement::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    states.transform *= getTransform();
    
    sf::RectangleShape placeHolder (sf::Vector2f(m_width, getHeight()));
    placeHolder.setFillColor(sf::Color::Transparent);
    placeHolder.setOutlineThickness(1);
    placeHolder.setOutlineColor(sf::Color::Red);

    target.draw(placeHolder, states);
}

void UIElement::onClick(int x, int y) {
    std::cout << "wrong click" << std::endl;
}

void UIElement::onHotkey() {}
