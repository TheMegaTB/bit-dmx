//
//  UIElement.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Element.hpp"


Element::Element(int width, int height) {
    m_width = width;
    m_height = height;
}

void Element::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    states.transform *= getTransform();
    
    sf::RectangleShape placeHolder (sf::Vector2f(getWidth(), getHeight()));
    placeHolder.setFillColor(sf::Color::Transparent);
    placeHolder.setOutlineThickness(1);
    placeHolder.setOutlineColor(sf::Color::Red);
    
    target.draw(placeHolder, states);
}
