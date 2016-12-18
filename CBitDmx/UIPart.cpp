//
//  UIPart.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIPart.hpp"


int UIPart::getHeight() const {
    return UIPartWidth / 4;
}

void UIPart::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {}
void UIPart::onMouseMove(int x, int y, sf::Mouse::Button mouseButton) {}
void UIPart::onMouseRelease(int x, int y, sf::Mouse::Button mouseButton) {}

void UIPart::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    states.transform *= getTransform();
    
    sf::RectangleShape placeHolder (sf::Vector2f(UIPartWidth, getHeight()));
    placeHolder.setFillColor(sf::Color::Transparent);
    placeHolder.setOutlineThickness(1);
    placeHolder.setOutlineColor(sf::Color::Red);
    
    target.draw(placeHolder, states);
}
