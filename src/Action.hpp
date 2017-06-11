//
//  Action.hpp
//  BitDmx
//
//  Created by Noah Peeters on 12/23/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Action_hpp
#define Action_hpp

#include <stdio.h>

#include <SFML/Graphics.hpp>

#include "json.hpp"
using json = nlohmann::json;

#include "Types.hpp"
#include "FadeCurve.hpp"

class UnvaluedAction {
public:
    UnvaluedAction();
    UnvaluedAction(json jsonData);

    void setFadeTime(sf::Time fadeTime) { m_fadeTime = fadeTime; };
    void setFadeCurve(FadeCurve fadeCurve) { m_fadeCurve = fadeCurve; };
    
    sf::Time getFadeTime() const { return m_fadeTime; };
    FadeCurve getFadeCurve() const { return m_fadeCurve; };
    
private:
    sf::Time m_fadeTime;
    FadeCurve m_fadeCurve;
};

class ValuedAction : public UnvaluedAction {
public:
    ValuedAction(ChannelValue value);
    ValuedAction(json jsonData);
    ValuedAction(UnvaluedAction baseAction, ChannelValue value);
    
    void setValue(std::string subname, ChannelValue value) { m_value = value; }
    
    ChannelValue getValue() const { return m_value; };
    
private:
    ChannelValue m_value;
};


#endif /* Action_hpp */
