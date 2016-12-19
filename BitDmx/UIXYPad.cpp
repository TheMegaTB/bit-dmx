//
//  UIXYPad.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIXYPad.hpp"

UIXYPad::UIXYPad(Stage* stage, ChannelAddress channelXAddress, ChannelAddress channelYAddress): UILabeledElement(stage, stage->UIPartWidth, stage->UIPartWidth) {
    setChannelAddress(channelXAddress, channelYAddress);
    
    m_xyPad = std::make_shared<XYPad>(0, 255, [this](double x, double y) -> void { this->activate(); }, [this]() -> void { this->deactivate(); }, stage->UIPartWidth, stage->UIPartWidth, m_stage->getFont());
    addPart(m_xyPad);
}

void UIXYPad::setChannelAddress(ChannelAddress channelXAddress, ChannelAddress channelYAddress) {
    m_channelXAddress = channelXAddress;
    m_channelYAddress = channelYAddress;
}

void UIXYPad::action() {
    m_stage->startFade(m_channelXAddress, m_fadeTime, m_xyPad->getXValue(), m_fadeCurve, m_id);
    m_stage->startFade(m_channelYAddress, m_fadeTime, m_xyPad->getYValue(), m_fadeCurve, m_id);
}
