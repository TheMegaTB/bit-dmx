//
//  UIController.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "ElementController.hpp"


ElementController::ElementController(std::vector<std::shared_ptr<Element>> uiElements, int width, int height) : Element(width, height) {
    m_elements = uiElements;
}

void ElementController::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {
    m_lastClickOn = findElementByXY(x, y);
    if (m_lastClickOn != -1) {
        sf::Vector2f position = m_elements[m_lastClickOn]->getPosition();
        m_elements[m_lastClickOn]->onMousePress(x - position.x, y - position.y, mouseButton);
    }
}


void ElementController::onMouseDrag(int x, int y, sf::Mouse::Button mouseButton) {
    if (m_lastClickOn != -1) {
        sf::Vector2f position = m_elements[m_lastClickOn]->getPosition();
        m_elements[m_lastClickOn]->onMouseDrag(x - position.x, y - position.y, mouseButton);
    }
}

void ElementController::onMouseRelease(int x, int y, sf::Mouse::Button mouseButton) {
    if (m_lastClickOn != -1) {
        sf::Vector2f position = m_elements[m_lastClickOn]->getPosition();
        m_elements[m_lastClickOn]->onMouseRelease(x - position.x, y - position.y, mouseButton);
        m_lastClickOn = -1;
    }
}

int ElementController::findElementByXY(int x, int y) {
    for (int i = 0; i < m_elements.size(); i++) {
        sf::Vector2f position = m_elements[i]->getPosition();
        if ((x >= position.x) && (x <= position.x + getWidth()) &&
            (y >= position.y) && (y <= position.y + m_elements[i]->getHeight())) {
            return i;
        }
    }
    return -1;
}

void ElementController::draw(sf::RenderTarget& target, sf::RenderStates states) const {
    states.transform *= getTransform();
    for (std::shared_ptr<Element> uiElement : m_elements) {
        target.draw(*uiElement, states);
    }
}


void ElementController::addElement(std::shared_ptr<Element> element) {
    m_elements.push_back(element);
}
