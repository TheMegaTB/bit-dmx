//
//  Label.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Label.hpp"

Label::Label(std::string caption, int width, int height, sf::Font font): Element(width, height) {
    setCaption(caption);
    m_font = font;
}

void Label::setCaption(std::string caption) {
    m_caption = caption;
}

void Label::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    states.transform *= getTransform();
    
//    sf::RectangleShape labelShape (sf::Vector2f(getWidth(), getHeight()));
//    labelShape.setFillColor(sf::Color::Yellow);
//    
//    labelShape.setOutlineThickness(1);
//    labelShape.setOutlineColor(sf::Color::Black);
//    
//    target.draw(labelShape, states);
    
    sf::Text caption = sf::Text(m_caption, m_font, 15);
    caption.setPosition((getWidth() - caption.getLocalBounds().width) / 2, (getHeight() - caption.getLocalBounds().height) / 2);
    caption.setFillColor(sf::Color::White);
    
    target.draw(caption, states);
}
