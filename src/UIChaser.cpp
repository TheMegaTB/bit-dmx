//
//  UIChaser.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIChaser.hpp"



ChaserStep::ChaserStep(Stage *stage, json jsonData) : m_actionGroup(stage) {
    if (!jsonData.count("actions") || !jsonData["actions"].is_object()) {
        std::cout << "Cannot load actions" << std::endl;
        std::cout << std::setw(4) << jsonData << std::endl;
        exit(1);
    }
    if (!jsonData.count("time") || !jsonData["time"].is_number()) {
        std::cout << "Cannot load chaser time (time)" << std::endl;
        std::cout << std::setw(4) << jsonData << std::endl;
        exit(1);
    }
    
    m_chaserTime = sf::milliseconds(jsonData["time"]);
    m_actionGroup = ValuedActionGroup(stage, jsonData["actions"]);
    
    if (jsonData.count("min_round") && jsonData["min_round"].is_number()) {
        m_minRound = jsonData["min_round"];
    } else {
        m_minRound = -1;
    }
    if (jsonData.count("round") && jsonData["round"].is_number()) {
        m_round = jsonData["round"];
    } else {
        m_round = -1;
    }
    if (jsonData.count("max_round") && jsonData["max_round"].is_number()) {
        m_maxRound = jsonData["max_round"];
    } else {
        m_maxRound = -1;
    }
    
    if (jsonData.count("activate") && jsonData["activate"].is_boolean()) {
        m_activate = jsonData["activate"];
    } else {
        m_activate = true;
    }
    if (jsonData.count("deactivate") && jsonData["deactivate"].is_boolean()) {
        m_deactivate = jsonData["deactivate"];
    } else {
        m_deactivate = true;
    }
}

bool ChaserStep::inRound(int round) {
    return (m_minRound == -1 || round >= m_minRound) &&
           (m_round    == -1 || round == m_round   ) &&
           (m_maxRound == -1 || round <= m_maxRound);
}


UIChaser::UIChaser(Stage* stage, std::vector<ChaserStep> chaserSteps): UISingleVChannel(stage, stage->UIElementWidth, stage->UIElementWidth / 4)  {
    m_chaserSteps = chaserSteps;
    
    m_toggle = std::make_shared<Toggle>("Play", stage->UIElementWidth, stage->UIElementWidth / 4, m_stage->getFont());
    m_toggle->onChange([this](bool isActivated) -> void {
        if (isActivated) {
            this->setValue("value", 255, SELF_ACTIVATION);
        } else {
            this->deactivateActivation(SELF_ACTIVATION);
        }
    });
    addElement(m_toggle);
}

void UIChaser::update() {
    if (m_virtualChannel.update(m_stage->getNow())) {
        m_toggle->setActivation(isActivated());
        if (isActivated()) {
            m_position = -1;
            m_round = -1;
            m_startTime = m_stage->getNow();
            next();
            m_toggle->setCaption("Pause");
        } else {
            m_toggle->setCaption("Play");
            deactivateStep(m_position);
        }
    }
    if (isActivated()) {
        int time = (m_stage->getNow() - m_startTime).asMilliseconds();
        if (time > m_chaserSteps[m_position].getChaserTime().asMilliseconds()) {
            next();
        }
    }
}

void UIChaser::next() {
    int old_position = m_position;
    do {
        m_position = (m_position + 1) % m_chaserSteps.size();
        if (m_position == 0) {
            m_round++;
        }
    } while (!m_chaserSteps[m_position].inRound(m_round + 1));
    
    if (old_position != -1) m_startTime = m_startTime + m_chaserSteps[old_position].getChaserTime();
    
    activateStep(m_position);
    if (old_position != -1) deactivateStep(old_position);
}


void UIChaser::activateStep(int id) {
    if (m_chaserSteps[id].activate()) {
        m_stage->activateActivationGroup(m_chaserSteps[id].getActionGroup());
    }
}

void UIChaser::deactivateStep(int id) {
    if (m_chaserSteps[id].deactivate()) {
        m_stage->deactivateActivationGroup(m_chaserSteps[id].getActionGroup());
    }
}
