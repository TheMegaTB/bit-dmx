//
//  Stage.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Stage.hpp"

#include "UISwitch.hpp"
#include "UIPushButton.hpp"
#include "UIChannel.hpp"
#include "UIXYPad.hpp"
#include "UIChaser.hpp"



Stage::Stage(std::string port, std::string fontPath, std::string stagePath, std::string uiPath) {
//    if (port == "") {
//        m_fakeInterface = true;
//        std::cout << "Using fake interface" << std::endl;
//    } else {
        m_fakeInterface = false;
        std::cout << "Using interface '" << port << "'" << std::endl;
//        reconnect();
        open_port(115200, port.c_str());
//        m_previousChannel = -1;
//        openChannel(1);
//    }
    
    m_font.loadFromFile(fontPath);
    m_lastClickOn = -1;
    m_mouseX = 0;
    m_mouseY = 0;

    std::ifstream stageInputFile(stagePath);
    json stageJson;
    stageInputFile >> stageJson;
    
    
    setName(stageJson["name"]);
    m_channels.resize(stageJson["size"]);
    
    for (ChannelAddress channel = 1; channel < m_channels.size(); channel++) {
        updateChannel(channel);
    }

    
    for (auto& fixture : stageJson["fixtures"]) {
        ChannelAddress baseAddress = fixture["channel"];
        std::string templateName = fixture["template"];
        std::string namePrefix = fixture["name"].get<std::string>() + ":";
        
        std::vector<std::string> channelNames = stageJson["fixture_templates"][templateName];
        for (int i = 0; i < channelNames.size(); i++) {
            m_namedChannels[namePrefix + channelNames[i]] = baseAddress + i;
        }
    }

    std::ifstream uiInputFile(uiPath);
    json uiJson;
    uiInputFile >> uiJson;

    for (json::iterator it = uiJson.begin(); it != uiJson.end(); ++it) {
        m_namedUIElements[it.key()] = m_ui_elements.size();
        auto uiElement = it.value();
        UIControlElementType type = (UIControlElementType)uiElement["type"].get<int>();
        
        switch (type) {
            case UIControlElementChaser: {
                std::shared_ptr<UIChaser> e = std::make_shared<UIChaser>(this, uiElement);
                addUiElement(e);
                break;

            }
            case UIControlElementSwitch: {
                std::shared_ptr<UISwitch> e = std::make_shared<UISwitch>(this, uiElement);
                addUiElement(e);
                break;
            }
            case UIControlElementPushButton: {
                std::shared_ptr<UIPushButton> e = std::make_shared<UIPushButton>(this, uiElement);
                addUiElement(e);
                break;
            }
            case UIControlElementChannel: {
                std::shared_ptr<UIChannel> e = std::make_shared<UIChannel>(this, uiElement);
                addUiElement(e);
                break;
            }
            case UIControlElementXYPad: {
                std::shared_ptr<UIXYPad> e = std::make_shared<UIXYPad>(this, uiElement);
                addUiElement(e);
                break;
            }
        }
        if (uiElement.count("caption")) {
            m_ui_elements.back()->setCaption(uiElement["caption"]);
        }
        if (uiElement.count("visible")) {
            m_ui_elements.back()->setVisibility(uiElement["visible"]);
        }
        if (uiElement.count("fade_time")) {
            m_ui_elements.back()->setFadeTime(sf::milliseconds(uiElement["fade_time"]));
        }
        if (uiElement.count("fade_curve")) {
            m_ui_elements.back()->setFadeCurve((FadeCurve)uiElement["fade_curve"].get<int>());
        }
    }

}

//////////////////
//   Channels   //
//////////////////
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

////////////////////
//   UIElements   //
////////////////////
void Stage::activateUIElement(int elementID) {
    m_ui_elements[elementID]->action();
}

void Stage::deactivateUIElement(int elementID) {
    for (ChannelAddress channelAddress = 0; channelAddress < m_channels.size(); channelAddress++) {
        m_channels[channelAddress].disableUIElement(elementID, m_currentTime);
    }
}

void Stage::chaserActivateUIElement(int elementID) {
    m_ui_elements[elementID]->chaserActivate();
}

void Stage::chaserDeactivateUIElement(int elementID) {
    m_ui_elements[elementID]->chaserDeactivate();
}



///////////////////
//      Get      //
///////////////////
ChannelValue Stage::getValue(ChannelAddress address) const {
    if (address < m_channels.size()) {
        return m_channels[address].getValue(m_currentTime);
    } else {
        return -1;
    }
}

bool Stage::inEditMode() {
    return m_editMode;
}

sf::Font Stage::getFont() {
    return m_font;
}

sf::Time Stage::getNow() {
    return m_currentTime;
}


std::string Stage::getName() {
    return  m_name;
}


int Stage::getChannel(std::string channelName) {
    return m_namedChannels[channelName];
}


std::vector<int> Stage::getChannels(std::vector<std::string> channelNames) {
    std::vector<int> result;
    result.reserve(channelNames.size());
    
    
    for (std::string channelName : channelNames) {
        result.push_back(getChannel(channelName));
    }
    return result;
}


int Stage::getUIElement(std::string elementName) {
    return m_namedUIElements[elementName];
}

///////////////////////
//     Configure     //
///////////////////////
int Stage::addUiElement(std::shared_ptr<UIControlElement> uiElement) {
    uiElement->setID(m_ui_elements.size());
    m_ui_elements.push_back(uiElement);
    return m_ui_elements.size() - 1;
}


void Stage::setCurrentTime(sf::Time currentTime) {
    m_currentTime = currentTime;
    for (std::shared_ptr<UIControlElement> uiElement : m_ui_elements) {
        uiElement->update();
    }
}

void Stage::setName(std::string name) {
    m_name = name;
}

///////////////////
//     Other     //
///////////////////
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

void Stage::openChannel(ChannelAddress address) {
//    if (address != m_previousChannel) {
//        m_previousChannel = address;
//        m_interface << (char)0x01; //Enter channel mode
//        char clow = address & 0xff;
//        char chigh = (address >> 8);
//        m_interface << chigh;
//        m_interface << clow;
//    }
}

bool Stage::updateChannel(ChannelAddress address) {
//    if (m_fakeInterface) {
//        std::cout << "C" << address << " -> " << (int)m_channels[address].getValue(m_currentTime) << std::endl;
//    } else {
//        openChannel(address);
//        m_interface << (char)0x00; //Enter value mode
//        m_interface << (char)m_channels[address].getValue(m_currentTime);
//    }
//    return true; //TODO implement
    write_dmx(address, m_channels[address].getValue(m_currentTime));
    return true;
}


////////////////////
//     Events     //
////////////////////
void Stage::onMousePress(int x, int y, sf::Mouse::Button mouseButton) {
    m_lastClickOn = findUIElementByXY(x, y);
    if (m_lastClickOn != -1) {
        sf::Vector2f position = m_ui_elements[m_lastClickOn]->getPosition();
        m_ui_elements[m_lastClickOn]->onMousePress(x - position.x, y - position.y, mouseButton);
    }
    if (m_editMode) {
        m_uiElementInEditMode = m_lastClickOn;
    }
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
//    if (key == sf::Keyboard::Escape) {
//        toggleEditMode();
//    } else {
    for (unsigned int i = 0; i < m_ui_elements.size(); i++) {
        m_ui_elements[i]->hotkeyWrapper(key);
    }
//    }
}

void Stage::onHotkeyRelease(sf::Keyboard::Key key) {
    for (unsigned int i = 0; i < m_ui_elements.size(); i++) {
        m_ui_elements[i]->hotkeyReleaseWrapper(key);
    }
}










int Stage::findUIElementByXY(int x, int y) {
    for (int i = 0; i < m_ui_elements.size(); i++) {
        sf::Vector2f position = m_ui_elements[i]->getPosition();
        if ((x >= position.x) && (x <= position.x + UIPartWidth) &&
            (y >= position.y) && (y <= position.y + m_ui_elements[i]->getHeight())) {
            return i;
        }
    }
    return -1;
}


void Stage::toggleEditMode() {
    m_editMode = !m_editMode;
    m_uiElementInEditMode = -1;
}

void Stage::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    states.transform *= getTransform();
    
    int viewportSize = m_editMode ? target.getSize().x - UIPartWidth - 2 * UIPartDistance : target.getSize().x;
    int numberPerRow = (viewportSize - UIPartDistance) / (UIPartWidth + UIPartDistance);
    
    if (numberPerRow > 0) {
        std::vector<int> height(numberPerRow);
        
        for (unsigned int i = 0; i < m_ui_elements.size(); i++) {
            if (!m_ui_elements[i]->isVisible()) {
                continue;
            }
            
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
    
//    if (m_editMode) {
//        if (m_uiElementInEditMode != -1) {
//            sf::Transformable a;
//            a.setPosition(target.getSize().x - UIPartWidth - 2 * UIPartDistance, 0);
//            
//            states.transform *= a.getTransform();
//            m_ui_elements[m_uiElementInEditMode]->drawEditor(target, states);
//        };
//    }
}
