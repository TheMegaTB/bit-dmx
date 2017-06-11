//
//  Stage.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Stage_hpp
#define Stage_hpp

class Stage;

enum ActionType {
    ActionTypeChannel = 0,
    ActionTypeUI
};

#include <stdio.h>
#include <vector>
#include <memory>
#include <fstream>
#include <iostream>
#include <fcntl.h>

#include "Channel.hpp"
#include "ActionGroup.hpp"

#include "UIControlElement.hpp"
#include "arduino-serial-dmx.hpp"

class Stage: public sf::Drawable, public sf::Transformable {
public:
    int UIElementWidth = 160;
    Stage(std::string port, std::string fontPath, std::string configPath);
    
    
    // interacting with the channels
    bool startFade(ChannelAddress address, ChannelValue value, sf::Time fadeTime, FadeCurve fadeCurve, int activationID);
    
    void activateActivationGroup(ValuedActionGroup actionGroup);
    void activateActivationGroup(UnvaluedActionGroup actionGroup, ChannelValue value);
    
    void deactivateActivationGroup(ValuedActionGroup actionGroup);
    void deactivateActivationGroup(UnvaluedActionGroup actionGroup);
    
    // get values
//    ChannelValue getValue(ChannelAddress address) const;
    sf::Font getFont();
    sf::Time getNow();
    std::string getName();
    ActionTarget stringIDToNumberID(std::string stringId);
    int getNextID();
    
    // configrue
    int addUiElement(std::string stringID, std::shared_ptr<UIControlElement> uiElement);
    void setName(std::string name);
    
    // other
    void setCurrentTime(sf::Time currentTime);
    void updateAllChannels();
    
    // events
    void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
    void onMouseDrag(int x, int y);
    void onMouseRelease(int x, int y, sf::Mouse::Button mouseButton);
    void onHotkey(sf::Keyboard::Key key);
    void onHotkeyRelease(sf::Keyboard::Key key);
    void onScroll(int delta);

private:
    //mouse
    int m_mouseX;
    int m_mouseY;
    int m_lastClickOn;
    sf::Mouse::Button m_lastClickButton;
    int m_yScroolPosition;
    
    // other
    sf::Time m_currentTime;
    sf::Font m_font;
    std::string m_name;
    std::string m_configPath;
    std::vector<std::vector<std::vector<int>>> m_pages;
    int m_page;
    int m_nextID;
    
    //stage data
    std::vector<Channel> m_channels;
    std::vector<std::shared_ptr<UIControlElement>> m_ui_elements;
    std::map<std::string, std::pair<ActionType, int>> m_stringIDs;

    // functions
    bool updateChannel(ChannelAddress address);
    int findUIElementByXY(int x, int y);
    
    void loadUIConfig(std::string path);
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
};

#endif /* Stage_hpp */
