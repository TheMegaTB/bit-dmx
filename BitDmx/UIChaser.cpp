//
//  UIChaser.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIChaser.hpp"

UIChaser::UIChaser(Stage* stage, json chaserData): UILabeledElement(stage, stage->UIPartWidth, stage->UIPartWidth)  {
    m_chaserData = chaserData["chaser"].get<std::vector<json>>();
    
    m_toggle = std::make_shared<Toggle>([this](bool isActivated) -> void {
        if (isActivated) {
            this->activate();
        } else {
            this->deactivate();
        }
    }, "Play", stage->UIPartWidth, stage->UIPartWidth / 4, m_stage->getFont());
    addPart(m_toggle);
}

void UIChaser::update() {
    if (m_isActivated) {
        int time = (m_stage->getNow() - m_startTime).asMilliseconds();
        if (time > m_chaserData[m_position]["time"].get<int>()) {
            next();
        }
    }
}

void UIChaser::next() {
    int old_position = m_position;
    do {
        m_position = (m_position + 1) % m_chaserData.size();
        if (m_position == 0) {
            m_round++;
        }
    } while (
        (m_chaserData[m_position].count("min_round") && (m_chaserData[m_position]["min_round"].get<int>() >= m_round + 1)) ||
             (m_chaserData[m_position].count("max_round") && (m_chaserData[m_position]["max_round"].get<int>() <= m_round + 1)) ||
             (m_chaserData[m_position].count("round") && (m_chaserData[m_position]["round"].get<int>() != m_round + 1)));
    
    
    
    m_startTime = m_startTime + sf::milliseconds(m_chaserData[old_position]["time"]);
    
    if (!m_chaserData[m_position].count("activate") || m_chaserData[m_position]["activate"]) {
        m_stage->chaserActivateUIElement(m_stage->getUIElement(m_chaserData[m_position]["name"]));
    }
    if (!m_chaserData[old_position].count("deactivate") || m_chaserData[old_position]["deactivate"]) {
        m_stage->chaserDeactivateUIElement(m_stage->getUIElement(m_chaserData[old_position]["name"]));
    }
}

void UIChaser::chaserActivate() {
    activate();
    m_toggle->setActivation(true);
}

void UIChaser::chaserDeactivate() {
    m_isActivated = false;
    m_toggle->setCaption("Play");
    m_toggle->setActivation(false);
}

void UIChaser::activate() {
    if (!m_isActivated) {
        m_position = 0;
        m_round = 0;
        m_isActivated = true;
        m_startTime = m_stage->getNow();
        m_stage->chaserActivateUIElement(m_stage->getUIElement(m_chaserData[m_position]["name"]));
        m_toggle->setCaption("Pause");
    }
}

void UIChaser::deactivate() {
    m_isActivated = false;
    m_toggle->setCaption("Play");
    m_toggle->setActivation(false);
    m_stage->chaserDeactivateUIElement(m_stage->getUIElement(m_chaserData[m_position]["name"]));

}
