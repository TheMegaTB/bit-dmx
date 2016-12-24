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
#include "UISlieder.hpp"
#include "UIXYPad.hpp"
#include "UIChaser.hpp"

#include "ActionGroup.hpp"


Stage::Stage(std::string port, std::string fontPath, std::string configPath) {
    // Connect to interface
    std::cout << "Using interface '" << port << "'" << std::endl;
    open_port(115200, port.c_str());
    
    //set basic information
    m_font.loadFromFile(fontPath);
    m_lastClickOn = -1;
    m_mouseX = 0;
    m_mouseY = 0;
    m_yScroolPosition = 0;
    m_configPath = configPath;
    m_pages = {{}};
    m_page = 0;

    // open stage config file
    std::ifstream stageInputFile(m_configPath + "/stage.json");
    json stageJson;
    stageInputFile >> stageJson;
    
    // load name and size
    setName("Untitled");
    if (stageJson.count("name") && stageJson["name"].is_string()) {
        setName(stageJson["name"]);
    }
    if (stageJson.count("size") && stageJson["size"].is_number()) {
        m_channels.resize(stageJson["size"]);
    }
    
    // reset all channels
    for (ChannelAddress channel = 1; channel < m_channels.size(); channel++) {
        updateChannel(channel);
    }
    
//    std::map<std::string, std::vector<std::string>> templates;
//    
//    try {
//        templates = stageJson["fixture_templates"].get<std::map<std::string, std::vector<std::string>>>();
//    } catch (std::exception& e) {
//        std::cout << "Cannot load templates";
//    }
    
    auto templates = stageJson["fixture_templates"];//TODO make safe

    //load fixtures
    if (stageJson.count("fixtures") && stageJson["fixtures"].is_array()) {
        for (auto& fixture : stageJson["fixtures"]) {
            if (!fixture.count("channel") || !fixture["channel"].is_number()) {
                std::cout << "Cannot load channel" << std::endl;
                std::cout << std::setw(4) << fixture << std::endl;
                continue;
            }
            if (!fixture.count("template") || !fixture["template"].is_string()) {
                std::cout << "Cannot load template" << std::endl;
                std::cout << std::setw(4) << fixture << std::endl;
                continue;
            }
            if (!fixture.count("name") || !fixture["name"].is_string()) {
                std::cout << "Cannot load name" << std::endl;
                std::cout << std::setw(4) << fixture << std::endl;
                continue;
            }
            
            ChannelAddress baseAddress = fixture["channel"];
            std::string templateName = fixture["template"];
            std::string namePrefix = fixture["name"].get<std::string>() + ":";
            
            if (templates.count(templateName)) {
                std::vector<std::string> channelNames = templates[templateName];
                for (int i = 0; i < channelNames.size(); i++) {
                    m_stringIDs[namePrefix + channelNames[i]] = std::make_pair(ActionTypeChannel, baseAddress + i);
                }
            } else {
                std::cout << "The is no template with the name " << templateName << std::endl;
            }
            
        }
    } else {
        std::cout << "Cannot find valid fixture data" << std::endl;
    }
    
    // load base ui
    loadUIConfig("ui.json");
}

void Stage::loadUIConfig(std::string name) {
    std::cout << "parsing ui config: " << m_configPath + "/" + name << std::endl;
    std::ifstream uiInputFile(m_configPath + "/" + name);
    json uiJson;
    uiInputFile >> uiJson;
    
    if (uiJson.count("import") && !uiJson["import"].is_array()) {
        for (auto importName : uiJson["import"]) {
            if (importName.is_string()) {
                loadUIConfig(importName);
            } else {
                std::cout << std::setw(4) << importName << " is not a valid name" << std::endl;
            }
        }
    } else {
        std::cout << "Cannot find import area" << std::endl;
    }
    
    std::cout << "loading ui config: " << m_configPath + "/" + name << std::endl;
    
    if (uiJson.count("data") && uiJson["data"].is_object()) {
        for (json::iterator it = uiJson["data"].begin(); it != uiJson["data"].end(); ++it) {
            m_stringIDs[it.key()] = std::make_pair(ActionTypeUI, m_ui_elements.size());
            auto uiElement = it.value();
            
            if (!uiElement.count("type") || !uiElement["type"].is_number()) {
                std::cout << "Cannot load type" << std::endl;
                std::cout << std::setw(4) << uiElement << std::endl;
                continue;
            }
            
            UIControlElementType type = (UIControlElementType)uiElement["type"].get<int>();
        
            
            switch (type) {
                case UIControlElementChaser: {
                    if (!uiElement.count("chaser") || !uiElement["chaser"].is_array()) {
                        std::cout << "Cannot load chaser" << std::endl;
                        std::cout << std::setw(4) << uiElement << std::endl;
                        continue;
                    }
                    std::vector<ChaserStep> chaserSteps;
                    chaserSteps.reserve(uiElement["chaser"].size());
                    for (json chaserStepData : uiElement["chaser"]) {
                        chaserSteps.push_back(ChaserStep(this, chaserStepData));
                    }
                    
                    
                    std::shared_ptr<UIChaser> e = std::make_shared<UIChaser>(this, chaserSteps);
                    addUiElement(it.key(), e);
                    break;
                    
                }
                case UIControlElementSwitch: {
                    if (!uiElement.count("actions") || !uiElement["actions"].is_object()) {
                        std::cout << "Cannot load actions" << std::endl;
                        std::cout << std::setw(4) << uiElement << std::endl;
                        continue;
                    }
                    std::shared_ptr<UISwitch> e = std::make_shared<UISwitch>(this, ValuedActionGroup(this, uiElement["actions"]));
                    addUiElement(it.key(), e);
                    break;
                }
                case UIControlElementPushButton: {
                    if (!uiElement.count("actions") || !uiElement["actions"].is_object()) {
                        std::cout << "Cannot load actions" << std::endl;
                        std::cout << std::setw(4) << uiElement << std::endl;
                        continue;
                    }
                    std::shared_ptr<UIPushButton> e = std::make_shared<UIPushButton>(this, ValuedActionGroup(this, uiElement["actions"]));
                    addUiElement(it.key(), e);
                    break;
                }
                case UIControlElementSlieder: {
                    if (!uiElement.count("actions") || !uiElement["actions"].is_object()) {
                        std::cout << "Cannot load actions" << std::endl;
                        std::cout << std::setw(4) << uiElement << std::endl;
                        continue;
                    }
                    std::shared_ptr<UISlieder> e = std::make_shared<UISlieder>(this, UnvaluedActionGroup(this, uiElement["actions"]));
                    addUiElement(it.key(), e);
                    break;
                }
                case UIControlElementXYPad: {
                    if (!uiElement.count("actionsX") || !uiElement["actionsX"].is_object()) {
                        std::cout << "Cannot load actionsX" << std::endl;
                        std::cout << std::setw(4) << uiElement << std::endl;
                        continue;
                    }
                    if (!uiElement.count("actionsY") || !uiElement["actionsY"].is_object()) {
                        std::cout << "Cannot load actionsY" << std::endl;
                        std::cout << std::setw(4) << uiElement << std::endl;
                        continue;
                    }
                    std::shared_ptr<UIXYPad> e = std::make_shared<UIXYPad>(this, UnvaluedActionGroup(this, uiElement["actionsX"]), UnvaluedActionGroup(this, uiElement["actionsY"]));
                    addUiElement(it.key(), e);
                    break;
                }
            }
            
            // load general data
            if (uiElement.count("caption") && uiElement["caption"].is_string()) {
                m_ui_elements.back()->setCaption(uiElement["caption"]);
            } else {
                m_ui_elements.back()->setCaption(it.key());
            }
            if (uiElement.count("activate")  && uiElement["activate"].is_object()) {
                for (json::iterator it = uiJson["activate"].begin(); it != uiJson["activate"].end(); ++it) {
                    
                    if (it.value().is_number()) {
                        m_ui_elements.back()->setValue(it.key(), it.value(), SELF_ACTIVATION);
                    } else {
                        std::cout << "Cannot activate " << it.key() << " without a number" << std::endl;
                    }
                }
            }
            if (uiElement.count("hotkey") && uiElement["hotkey"].is_number()) {
                m_ui_elements.back()->setHotkey((sf::Keyboard::Key) uiElement["hotkey"].get<int>());
            }
            
            // load page and group
            int page = -1;
            int group = -1;
            if (uiElement.count("group") && uiElement["group"].is_number()) {
                group = uiElement["group"];
            }
            if (uiElement.count("group") && uiElement["group"].is_number()) {
                group = uiElement["group"];
            }
            
            // add to page/group
            if (page != -1 || group != -1) {
                if (page == -1) page = 0;
                if (group == -1) group = 0;
                
                if (page >= m_pages.size()) m_pages.resize(page + 1);
                if (group >= m_pages[page].size()) m_pages[page].resize(group + 1);
                
                m_pages[page][group].push_back(m_ui_elements.size()-1);
            }
        }
    } else {
        std::cout << "Cannot find data area" << std::endl;
    }
}

//////////////////
//   Channels   //
//////////////////
bool Stage::startFade(ChannelAddress address, ChannelValue value, sf::Time fadeTime, FadeCurve fadeCurve, int activationID) {
    if (address < m_channels.size()) {
        m_channels[address].startFade(m_currentTime, fadeTime, value, fadeCurve, activationID);
        return true;
    } else {
        return false;
    }
}

void Stage::activateActivationGroup(ValuedActionGroup actionGroup) {
    for (std::pair<ActionTarget, ValuedAction> action : actionGroup.get()) {
        if (action.first.actionType == ActionTypeChannel) {
            startFade(action.first.address, action.second.getValue(), action.second.getFadeTime(), action.second.getFadeCurve(), actionGroup.getID());
        } else {
            m_ui_elements[action.first.address]->startFade(action.first.subname, action.second.getFadeTime(), action.second.getValue(), action.second.getFadeCurve(), actionGroup.getID());
        }
    }
}

void Stage::activateActivationGroup(UnvaluedActionGroup actionGroup, ChannelValue value) {
    for (std::pair<ActionTarget, UnvaluedAction> action : actionGroup.get()) {
        if (action.first.actionType == ActionTypeChannel) {
            startFade(action.first.address, value, action.second.getFadeTime(), action.second.getFadeCurve(), actionGroup.getID());
        } else {
            m_ui_elements[action.first.address]->startFade(action.first.subname, action.second.getFadeTime(), value, action.second.getFadeCurve(), actionGroup.getID());
        }
    }
}

void Stage::deactivateActivationGroup(ValuedActionGroup actionGroup) {
    for (ChannelAddress channelAddress = 0; channelAddress < m_channels.size(); channelAddress++) {
        m_channels[channelAddress].deactivateActivation(m_currentTime, actionGroup.getID());
    }
    for (int UIElementID = 0; UIElementID < m_ui_elements.size(); UIElementID++) {
        m_ui_elements[UIElementID]->deactivateActivation(actionGroup.getID());
    }
}

void Stage::deactivateActivationGroup(UnvaluedActionGroup actionGroup) {
    for (ChannelAddress channelAddress = 0; channelAddress < m_channels.size(); channelAddress++) {
        m_channels[channelAddress].deactivateActivation(m_currentTime, actionGroup.getID());
    }
    for (int UIElementID = 0; UIElementID < m_ui_elements.size(); UIElementID++) {
        m_ui_elements[UIElementID]->deactivateActivation(actionGroup.getID());
    }
}


///////////////////
//      Get      //
///////////////////
//ChannelValue Stage::getValue(ChannelAddress address) const {
//    if (address < m_channels.size()) {
//        return m_channels[address].getValue();
//    } else {
//        return -1;
//    }
//}

sf::Font Stage::getFont() {
    return m_font;
}

sf::Time Stage::getNow() {
    return m_currentTime;
}


std::string Stage::getName() {
    return  m_name;
}


ActionTarget Stage::stringIDToNumberID(std::string stringId) {
    int split = stringId.find(":");
    
    if (split == -1) {
        std::cout << stringId << " is not a valid name!" << std::endl;
        exit(1);
    }
    
    std::string basename = stringId.substr(0, split);
    std::string subname = stringId.substr(split + 1, stringId.length() - split - 1);

    
    if (m_stringIDs.count(stringId)) {
        std::pair<ActionType, int> data = m_stringIDs[stringId];
        return ActionTarget(data.first, data.second, basename, subname);
    } else if (m_stringIDs.count(basename)) {
        std::pair<ActionType, int> data = m_stringIDs[basename];
        return ActionTarget(data.first, data.second, basename, subname);
    } else {
        std::cout << stringId << " or " << basename << " is not a valid name!" << std::endl;
        exit(1);
    }
}


int Stage::getNextID() {
    return m_nextID++;
}

///////////////////////
//     Configure     //
///////////////////////
int Stage::addUiElement(std::string stringID, std::shared_ptr<UIControlElement> uiElement) {
    m_stringIDs[stringID] = (std::make_pair(ActionTypeUI, m_ui_elements.size()));
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
void Stage::updateAllChannels() {
    for (ChannelAddress channelAddress = 0; channelAddress < m_channels.size(); channelAddress++) {
        if (m_channels[channelAddress].update(m_currentTime)) {
            updateChannel(channelAddress);
        }
    }
}

bool Stage::updateChannel(ChannelAddress address) {
    std::cout << "C" << address << " -> " << (int)m_channels[address].getValue() << std::endl;
    write_dmx(address, m_channels[address].getValue());
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
}


void Stage::onMouseDrag(int x, int y) {
    if (m_lastClickOn != -1) {
        sf::Vector2f position = m_ui_elements[m_lastClickOn]->getPosition();
        m_ui_elements[m_lastClickOn]->onMouseDrag(x - position.x, y - position.y, m_lastClickButton);
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
    for (unsigned int i = 0; i < m_ui_elements.size(); i++) {
        m_ui_elements[i]->hotkeyWrapper(key);
    }
}

void Stage::onHotkeyRelease(sf::Keyboard::Key key) {
    for (unsigned int i = 0; i < m_ui_elements.size(); i++) {
        m_ui_elements[i]->hotkeyReleaseWrapper(key);
    }
}

void Stage::onScroll(int delta) {
    m_yScroolPosition += delta;
    if (m_yScroolPosition > 0) {
        m_yScroolPosition = 0;
    }
}





int Stage::findUIElementByXY(int x, int y) {
    for (int i = 0; i < m_ui_elements.size(); i++) {
        sf::Vector2f position = m_ui_elements[i]->getPosition();
        if ((x >= position.x) && (x <= position.x + UIElementWidth) &&
            (y >= position.y) && (y <= position.y + m_ui_elements[i]->getHeight())) {
            return i;
        }
    }
    return -1;
}


void Stage::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    states.transform *= getTransform();
    
    int viewportSize = target.getSize().x;
    int numberPerRow = (viewportSize - UIElementDistance) / (UIElementWidth + UIElementDistance);
    int used_size = numberPerRow * (UIElementWidth + UIElementDistance) - UIElementDistance;
    int xOffset = (viewportSize-used_size)/2;
    
    if (numberPerRow > 0) {
        int y_offset = 0;
        
        for (std::vector<int> group : m_pages[m_page]) {
            std::vector<int> height(numberPerRow);
            
            for (unsigned int j = 0; j < group.size(); j++) {
                unsigned int i = group[j];
                unsigned int smallestHeight = height[0];
                unsigned int column = 0;
    
                for (unsigned int tmp_column = 1; tmp_column < numberPerRow; tmp_column++) {
                    if (height[tmp_column] < smallestHeight) {
                        smallestHeight = height[tmp_column];
                        column = tmp_column;
                    }
                }
                m_ui_elements[i]->setPosition(xOffset + column * (UIElementWidth + UIElementDistance), y_offset + height[column] + UIElementDistance + m_yScroolPosition);
    
                height[column] += m_ui_elements[i]->getHeight() + UIElementDistance;
    
                target.draw(*m_ui_elements[i]);
            }
            y_offset += *std::max_element(height.begin(), height.end()) + UIElementDistance * 6;
            
        }
    }
}
