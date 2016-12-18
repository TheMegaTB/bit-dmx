//
//  UIElement.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIControlElement.hpp"
#include <iostream>


UIControlElement::UIControlElement(Stage* stage, int width, int height) : UIController(width, height) {
    m_stage = stage;
    m_fadeTime = sf::seconds(1);
    m_fadeCurve = FadeCurve::linear;
}



void UIControlElement::setID(int id) {
    m_id = id;
}

void UIControlElement::setFadeTime(sf::Time fadeTime) {
    m_fadeTime = fadeTime;
}

void UIControlElement::setFadeCurve(FadeCurve fadeCurve) {
    m_fadeCurve = fadeCurve;
}

void UIControlElement::activate() {
    m_isActivated = true;
    m_stage->activateUIElement(m_id);
}

void UIControlElement::deactivate() {
    m_isActivated = false;
    m_stage->deactivateUIElement(m_id);
}

void UIControlElement::drawEditor(sf::RenderTarget& target, sf::RenderStates states) const {
    std::cout << "Edit" << std::endl;
}
