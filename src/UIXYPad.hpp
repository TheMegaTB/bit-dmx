//
//  UIXYPad.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIXYPad_hpp
#define UIXYPad_hpp

#include <stdio.h>

#include "UIControlElement.hpp"

class UIXYPad : public UIControlElement {
public:
    UIXYPad(Stage* stage, UnvaluedActionGroup actionGroupX, UnvaluedActionGroup actionGroupY);
    
    virtual void setValue(std::string subname, ChannelValue value, int activationID);
    virtual void startFade(std::string subname, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve, int activationID);
    virtual void deactivateActivation(int activationID);
    
    virtual bool isActivated() { return m_virtualChannelX.getValue() >= 0 || m_virtualChannelY.getValue() >= 0; };
    virtual void update();
private:
    std::shared_ptr<XYPad> m_xyPad;
    UnvaluedActionGroup m_actionGroupX;
    UnvaluedActionGroup m_actionGroupY;
    
    Channel m_virtualChannelX;
    Channel m_virtualChannelY;
};

#endif /* UIXYPad_hpp */
