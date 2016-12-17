//
//  Switch.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Switch.hpp"

Switch::Switch(Stage* stage, std::string name, std::vector<int> channelGroups): UIElementActivateble(stage) {
    setName(name);
    m_channelGroups = channelGroups;
    
    channelValues.resize(channelGroups.size());
    
    for (int i = 0; i < m_channelGroups.size(); i++) {
        channelValues[i].resize(m_stage->getChannelGroup(channelGroups[i])->getChannelNumber());
    }
}

void Switch::setName(std::string name) {
    m_name = name;
    m_uiName = m_stage->getText(m_name);
    m_uiName.setPosition((UIElementWidth - m_uiName.getLocalBounds().width) / 2, (getHeight() - m_uiName.getLocalBounds().height) / 2);
    m_uiName.setFillColor(sf::Color::Black);
}

void Switch::activate() {
    UIElementActivateble::activate();
//    action();
    std::cout << m_name << " activated" << std::endl;
}

void Switch::deactivate() {
    UIElementActivateble::deactivate();
    std::cout << m_name << " deactivated" << std::endl;
}

void Switch::action() {
    for (int i = 0; i < m_channelGroups.size(); i++) {
        m_stage->startFadeForChannelGroup(m_channelGroups[i], m_fadeTime, channelValues[i], m_fadeCurve, m_id);
    }
}


void Switch::draw(sf::RenderTarget& target, sf::RenderStates states) const {
    states.transform *= getTransform();
    
    sf::RectangleShape placeHolder (sf::Vector2f(m_width, getHeight()));
    if (m_isActivated) {
        placeHolder.setFillColor(sf::Color::Green);
    } else {
        placeHolder.setFillColor(sf::Color::Yellow);
    }
    placeHolder.setOutlineThickness(1);
    placeHolder.setOutlineColor(sf::Color::Red);
    
    target.draw(placeHolder, states);
    target.draw(m_uiName, states);
}
