//
//  Stage.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include <iostream>

#include "Stage.hpp"

Stage::Stage(int universeSize, std::string fontPath) {
    m_channels.resize(universeSize);
    m_font.loadFromFile(fontPath);
}

bool Stage::setValue(ChannelAddress address, ChannelValue value, int uiElementID) {
    if (address < m_channels.size()) {
        m_channels[address].setValue(value, uiElementID);
        return true;
    } else {
        return false;
    }
}

bool Stage::startFade(ChannelAddress address, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve, int uiElementID) {
    if (address < m_channels.size()) {
        m_channels[address].startFade(m_currentTime, fadeTime, value, fadeCurve, uiElementID);
        return true;
    } else {
        return false;
    }
}

void Stage::setValueForChannelGroup(int id, std::vector<ChannelValue> values, int uiElementID) {
    if (id < m_channelGroups.size()) {
        m_channelGroups[id].setValue(values, uiElementID);
        return true;
    } else {
        return false;
    }
}

void Stage::startFadeForChannelGroup(int id, sf::Time fadeTime, std::vector<ChannelValue> values, FadeCurve fadeCurve, int uiElementID) {
    if (id < m_channelGroups.size()) {
        m_channelGroups[id].startFade(fadeTime, values, fadeCurve, uiElementID);
        return true;
    } else {
        return false;
    }
}

ChannelValue Stage::getValue(ChannelAddress address) const {
    if (address < m_channels.size()) {
        return m_channels[address].getValue(m_currentTime);
    } else {
        return -1;
    }
}

void Stage::onClick(int x, int y) {
    for (UIElementWrapper uiElementWrapper : m_ui_elements) {
        sf::Vector2f position = uiElementWrapper.uiElement->getPosition();
        if ((x >= position.x) && (x <= position.x + UIElementWidth) &&
            (y >= position.y) && (y <= position.y + uiElementWrapper.uiElement->getHeight())) {
            uiElementWrapper.onClick(x - position.x, y - position.y);
        }
    }
}

void Stage::onHotkey(sf::Keyboard::Key key) {
    if (key == sf::Keyboard::Escape) {
        m_editMode = !m_editMode;
    } else {
        for (unsigned int i = 0; i < m_ui_elements.size(); i++) {
            m_ui_elements[i].onHotkey(key);
        }
    }
}

void Stage::addUiElement(std::shared_ptr<UIElement> uiElement) {
    uiElement->setID(m_ui_elements.size());
    m_ui_elements.push_back(UIElementWrapper(uiElement));
}

void Stage::addChannelGroup(ChannelGroup channelGroup) {
    m_channelGroups.push_back(channelGroup);
}

void Stage::addFixture(Fixture fixture) {
    m_fixtures.push_back(fixture);
}

ChannelGroup* Stage::getChannelGroup(int id) {
    return &m_channelGroups[id];
}

void Stage::setCurrentTime(sf::Time currentTime) {
    m_currentTime = currentTime;
}

bool Stage::inEditMode() {
    return m_editMode;
}

void Stage::activateUIElement(int elementID) {
//    m_ui_activation_order.push_back(elementID);
//    updateUIElements();
    m_ui_elements[elementID].uiElement->action();
}

void Stage::deactivateUIElement(int elementID) {
//    removeUIElement(elementID);
//    updateUIElements();
    for (ChannelAddress channelAddress = 0; channelAddress < m_channels.size(); channelAddress++) {
        m_channels[channelAddress].disableUIElement(elementID, m_currentTime);
    }
}

void Stage::updateUIElements() { //TODO update once before update all channels
//    for (ChannelAddress channelAddress = 0; channelAddress < m_channels.size(); channelAddress++) {
//        m_channels[channelAddress].setValue(0);
//    }
//    for (unsigned int i = 0; i < m_ui_activation_order.size(); i++) {
//        m_ui_elements[m_ui_activation_order[i]].uiElement->action();
//    }
}

bool Stage::updateAllChannels() {
    bool result = false;
    for (ChannelAddress channelAddress = 0; channelAddress < m_channels.size(); channelAddress++) {
        ChannelValue currentValue = m_channels[channelAddress].getValue(m_currentTime);
        if (currentValue != m_channels[channelAddress].getInterfaceValue()) {
            m_channels[channelAddress].setInterfaceValue(currentValue);
            updateChannel(channelAddress);
        }
    }
}

bool Stage::updateChannel(ChannelAddress address) {
    std::cout << "C" << address << " -> " << (int)m_channels[address].getValue(m_currentTime) << std::endl;
    return true; //TODO implement
}


sf::Text Stage::getText(std::string text) {
    return sf::Text (text, m_font, 12);
}

sf::Time Stage::getNow() {
    return m_currentTime;
}

void Stage::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    states.transform *= getTransform();
    
    int viewportSize = m_editMode ? target.getSize().x - UIElementWidth - 2 * UIElementDistance : target.getSize().x;
    int numberPerRow = (viewportSize - UIElementDistance) / (UIElementWidth + UIElementDistance);
    
    if (numberPerRow > 0) {
        std::vector<int> height(numberPerRow);
        
        for (unsigned int i = 0; i < m_ui_elements.size(); i++) {
            unsigned int smallestHeight = height[0];
            unsigned int column = 0;
            
            for (unsigned int tmp_column = 1; tmp_column < numberPerRow; tmp_column++) {
                if (height[tmp_column] < smallestHeight) {
                    smallestHeight = height[tmp_column];
                    column = tmp_column;
                }
            }
            
            m_ui_elements[i].uiElement->setPosition(UIElementDistance + column * (UIElementWidth + UIElementDistance), height[column] + UIElementDistance);
            
            height[column] += m_ui_elements[i].uiElement->getHeight() + UIElementDistance;
            
            target.draw(*m_ui_elements[i].uiElement);
        }
    }
}
