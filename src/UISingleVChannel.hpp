//
//  UISingleVChannel.hpp
//  BitDmx
//
//  Created by Noah Peeters on 12/23/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UISingleVChannel_hpp
#define UISingleVChannel_hpp

#include <stdio.h>

#include "UIControlElement.hpp"

class UISingleVChannel : public UIControlElement {
public:
    UISingleVChannel(Stage* stage, int width, int height) : UIControlElement(stage, width, height), m_virtualChannel(-1) {};
    
    virtual void setValue(std::string subname, ChannelValue value, int activationID);
    virtual void startFade(std::string subname, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve, int activationID);
    virtual void deactivateActivation(int activationID);
    
    virtual void onHotkey();
    
    virtual bool isActivated() { return m_virtualChannel.getValue() >= 0; };
protected:
    Channel m_virtualChannel;
};

#endif /* UISingleVChannel_hpp */
