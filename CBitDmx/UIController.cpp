//
//  UIController.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIController.hpp"



UIController::UIController(std::vector<std::shared_ptr<UIPart>> uiParts) {
    m_uiParts = uiParts;
}

void UIController::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {
    for (int i = 0; i < m_uiParts.size(); i++) {
        sf::Vector2f position = m_uiParts[i]->getPosition();
        if ((x >= position.x) && (x <= position.x + UIPartWidth) &&
            (y >= position.y) && (y <= position.y + m_uiParts[i]->getHeight())) {
            m_uiParts[i]->onMousePress(x - position.x, y - position.y, mouseButton);
            m_lastClickOn = i;
            return;
        }
    }
    m_lastClickOn = -1;
}


void UIController::onMouseMove(int x, int y, sf::Mouse::Button mouseButton) {
    if (m_lastClickOn != -1) {
        sf::Vector2f position = m_uiParts[m_lastClickOn]->getPosition();
        m_uiParts[m_lastClickOn]->onMouseMove(x - position.x, y - position.y, mouseButton);
    }
}

void UIController::onMouseRelease(int x, int y, sf::Mouse::Button mouseButton) {
    if (m_lastClickOn != -1) {
        sf::Vector2f position = m_uiParts[m_lastClickOn]->getPosition();
        m_uiParts[m_lastClickOn]->onMouseRelease(x - position.x, y - position.y, mouseButton);
        m_lastClickOn = -1;
    }
}

void UIController::draw(sf::RenderTarget& target, sf::RenderStates states) const {
    states.transform *= getTransform();
    for (std::shared_ptr<UIPart> uiPart : m_uiParts) {
        target.draw(*uiPart, states);
    }
}
