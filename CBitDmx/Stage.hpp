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

#include <stdio.h>
#include <vector>
#include <memory>

#include "Channel.hpp"
#include "ChannelGroup.hpp"
#include "Fixture.hpp"

#include "UIControlElement.hpp"

class Stage: public sf::Drawable, public sf::Transformable {
public:
    Stage(int universeSize, std::string fontPath);
    bool setValue(ChannelAddress address, ChannelValue value, int uiElementID);
    bool startFade(ChannelAddress address, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve, int uiElementID);
    
    void setValueForChannelGroup(int id, std::vector<ChannelValue> values, int uiElementID);
    void startFadeForChannelGroup(int id, sf::Time fadeTime, std::vector<ChannelValue> values, FadeCurve fadeCurve, int uiElementID);
    
    ChannelValue getValue(ChannelAddress address) const;
    
    bool updateAllChannels();
    
    sf::Font getFont();
    sf::Time getNow();
    
    void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
    void onMouseMove(int x, int y);
    void onMouseRelease(int x, int y, sf::Mouse::Button mouseButton);
    void onHotkey(sf::Keyboard::Key key);
    
    void addUiElement(std::shared_ptr<UIControlElement> uiElement);
    void addChannelGroup(ChannelGroup channelGroup);
    void addFixture(Fixture fixture);
    
    ChannelGroup* getChannelGroup(int id);
    
    void setCurrentTime(sf::Time currentTime);
    bool inEditMode();
    void activateUIElement(int elementID);
    void deactivateUIElement(int elementID);
private:
    // Edit mode
    bool m_editMode;
    int m_mouseX;
    int m_mouseY;
    
    //mouse
    int m_lastClickOn;
    sf::Mouse::Button m_lastClickButton;
    
    // other
    sf::Time m_currentTime;
    sf::Font m_font;
    
    //stage data
    std::vector<Channel> m_channels;
    std::vector<std::shared_ptr<UIControlElement>> m_ui_elements;
    std::vector<ChannelGroup> m_channelGroups;
    std::vector<Fixture> m_fixtures;

    
    bool updateChannel(ChannelAddress address);
    
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
};

#endif /* Stage_hpp */
