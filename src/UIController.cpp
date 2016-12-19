//
//  UIController.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIController.hpp"


UIController::UIController(std::vector<std::shared_ptr<UIPart>> uiParts, int width, int height) : UIPart(width, height) {
    m_parts = uiParts;
}

void UIController::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {
    m_lastClickOn = findPartByXY(x, y);
    if (m_lastClickOn != -1) {
        sf::Vector2f position = m_parts[m_lastClickOn]->getPosition();
        m_parts[m_lastClickOn]->onMousePress(x - position.x, y - position.y, mouseButton);
    }
}


void UIController::onMouseMove(int x, int y, sf::Mouse::Button mouseButton) {
    if (m_lastClickOn != -1) {
        sf::Vector2f position = m_parts[m_lastClickOn]->getPosition();
        m_parts[m_lastClickOn]->onMouseMove(x - position.x, y - position.y, mouseButton);
    }
}

void UIController::onMouseRelease(int x, int y, sf::Mouse::Button mouseButton) {
    if (m_lastClickOn != -1) {
        sf::Vector2f position = m_parts[m_lastClickOn]->getPosition();
        m_parts[m_lastClickOn]->onMouseRelease(x - position.x, y - position.y, mouseButton);
        m_lastClickOn = -1;
    }
}

int UIController::findPartByXY(int x, int y) {
    for (int i = 0; i < m_parts.size(); i++) {
        sf::Vector2f position = m_parts[i]->getPosition();
        if ((x >= position.x) && (x <= position.x + getWidth()) &&
            (y >= position.y) && (y <= position.y + m_parts[i]->getHeight())) {
            return i;
        }
    }
    return -1;
}

void UIController::draw(sf::RenderTarget& target, sf::RenderStates states) const {
    states.transform *= getTransform();
    for (std::shared_ptr<UIPart> uiPart : m_parts) {
        target.draw(*uiPart, states);
    }
}


void UIController::addPart(std::shared_ptr<UIPart> part) {
    m_parts.push_back(part);
}
