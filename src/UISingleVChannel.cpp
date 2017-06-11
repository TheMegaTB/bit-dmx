//
//  UISingleVChannel.cpp
//  BitDmx
//
//  Created by Noah Peeters on 12/23/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UISingleVChannel.hpp"

void UISingleVChannel::setValue(std::string subname, ChannelValue value, int activationID) {
    if (subname == "value") {
        m_virtualChannel.setValue(value, activationID);
    } else {
        std::cout << "There is no virtual channel named " << subname << " in a UISingleVChannel!" << std::endl;
    }
}

void UISingleVChannel::startFade(std::string subname, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve, int activationID) {
    if (subname == "value") {
        m_virtualChannel.startFade(m_stage->getNow(), fadeTime, value, fadeCurve, activationID);
    } else {
        std::cout << "There is no virtual channel named " << subname << " in a UISingleVChannel!" << std::endl;
    }
}

void UISingleVChannel::deactivateActivation(int activationID) {
    m_virtualChannel.deactivateActivation(m_stage->getNow(), activationID);
}

void UISingleVChannel::onHotkey() {
    if (isActivated()) {
        deactivateActivation(SELF_ACTIVATION);
    } else {
        setValue("value", 255, SELF_ACTIVATION);
    }
}
