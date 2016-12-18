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
    m_lastClickOn = -1;
    m_mouseX = 0;
    m_mouseY = 0;
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

void Stage::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {
    m_lastClickButton = mouseButton;
    for (int i = 0; i < m_ui_elements.size(); i++) {
        sf::Vector2f position = m_ui_elements[i]->getPosition();
        if ((x >= position.x) && (x <= position.x + UIPartWidth) &&
            (y >= position.y) && (y <= position.y + m_ui_elements[i]->getHeight())) {
            m_ui_elements[i]->onMousePress(x - position.x, y - position.y, mouseButton);
            m_lastClickOn = i;
            return;
        }
    }
    m_lastClickOn = -1;
}


void Stage::onMouseMove(int x, int y) {
    if (m_lastClickOn != -1) {
        sf::Vector2f position = m_ui_elements[m_lastClickOn]->getPosition();
        m_ui_elements[m_lastClickOn]->onMouseMove(x - position.x, y - position.y, m_lastClickButton);
    }
    m_mouseX = x;
    m_mouseY = y;
}

void Stage::onMouseRelease(int x, int y, sf::Mouse::Button mouseButton) {
    if (m_lastClickOn != -1) {
        sf::Vector2f position = m_ui_elements[m_lastClickOn]->getPosition();
        m_ui_elements[m_lastClickOn]->onMouseRelease(x - position.x, y - position.y, mouseButton);
        m_lastClickOn = -1;
    }
}

void Stage::onHotkey(sf::Keyboard::Key key) {
    if (key == sf::Keyboard::Escape) {
        m_editMode = !m_editMode;
    } else {
        for (unsigned int i = 0; i < m_ui_elements.size(); i++) {
            m_ui_elements[i]->hotkeyWrapper(key);
        }
    }
}

void Stage::addUiElement(std::shared_ptr<UIControlElement> uiElement) {
    uiElement->setID(m_ui_elements.size());
    m_ui_elements.push_back(uiElement);
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
    m_ui_elements[elementID]->action();
}

void Stage::deactivateUIElement(int elementID) {
    for (ChannelAddress channelAddress = 0; channelAddress < m_channels.size(); channelAddress++) {
        m_channels[channelAddress].disableUIElement(elementID, m_currentTime);
    }
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


sf::Font Stage::getFont() {
    return m_font;
}

sf::Time Stage::getNow() {
    return m_currentTime;
}

void Stage::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    states.transform *= getTransform();
    
    int viewportSize = m_editMode ? target.getSize().x - UIPartWidth - 2 * UIPartDistance : target.getSize().x;
    int numberPerRow = (viewportSize - UIPartDistance) / (UIPartWidth + UIPartDistance);
    
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
            
            m_ui_elements[i]->setPosition(UIPartDistance + column * (UIPartWidth + UIPartDistance), height[column] + UIPartDistance);
            
            height[column] += m_ui_elements[i]->getHeight() + UIPartDistance;
            
            target.draw(*m_ui_elements[i]);
        }
    }
}
