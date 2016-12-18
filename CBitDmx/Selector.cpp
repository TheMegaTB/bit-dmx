//
//  Selector.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Selector.hpp"

Selector::Selector(std::function<void(bool)> changeCallback, std::vector<std::string> options, int width, int height, sf::Font font): UIController(width, height) {
    m_changeCallback = changeCallback;
    m_currentPosition = 0;
    m_options = options;
    
    m_buttonLeft = std::make_shared<Button>([this](bool x) -> void { this->next(); }, "<<", height, height, font);
    m_buttonLeft->setPosition(0, 0);
    
    m_buttonRight = std::make_shared<Button>([this](bool x) -> void { this->previous(); }, "<<", height, height, font);
    m_buttonLeft->setPosition(height, 0);
    
    m_label = std::make_shared<Label>(m_options[m_currentPosition], width - 2*height,height, font);
    m_buttonLeft->setPosition(width - height, 0);
    
    addPart(m_buttonLeft);
    addPart(m_buttonRight);
    addPart(m_label);
}

void Selector::select(int position) {
    m_currentPosition = position % m_options.size();
    m_label->setCaption(m_options[m_currentPosition]);
}

void Selector::next() {
    select(m_currentPosition + 1);
}

void Selector::previous() {
    select(m_currentPosition - 1);
}
