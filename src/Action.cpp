//
//  Action.cpp
//  BitDmx
//
//  Created by Noah Peeters on 12/23/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Action.hpp"



UnvaluedAction::UnvaluedAction() {
    setFadeTime(sf::Time::Zero);
    setFadeCurve(linear);
}

UnvaluedAction::UnvaluedAction(json jsonData) : UnvaluedAction() {
    if (jsonData.count("fade_time") && jsonData["fade_time"].is_number()) {
        setFadeTime(sf::milliseconds(jsonData["fade_time"]));
    }
    if (jsonData.count("fade_curve") && jsonData["fade_curve"].is_number()) {
        setFadeCurve((FadeCurve)jsonData["fade_curve"].get<int>());
    }
}

ValuedAction::ValuedAction(ChannelValue value) {
    setFadeCurve(linear);
    setFadeTime(sf::Time::Zero);
}

ValuedAction::ValuedAction(UnvaluedAction baseAction, ChannelValue value) {
    m_value = value;
    setFadeTime(baseAction.getFadeTime());
    setFadeCurve(baseAction.getFadeCurve());
}

ValuedAction::ValuedAction(json jsonData) : ValuedAction(0) {
    if (jsonData.count("value") && jsonData["value"].is_number()) {
        m_value = jsonData["value"];
        
        if (jsonData.count("fade_time") && jsonData["fade_time"].is_number()) {
            setFadeTime(sf::milliseconds(jsonData["fade_time"]));
        }
        if (jsonData.count("fade_curve") && jsonData["fade_curve"].is_number()) {
            setFadeCurve((FadeCurve)jsonData["fade_curve"].get<int>());
        }
    } else {
        std::cout << "Cannot create ValuedAction without value" << std::endl;
        std::cout << std::setw(4) << jsonData << std::endl;
        exit(1);
    }
}
