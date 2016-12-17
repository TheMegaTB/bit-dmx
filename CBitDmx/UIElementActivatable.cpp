//
//  UIElementActivatable.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UIElementActivatable.hpp"



UIElementActivateble::UIElementActivateble(Stage* stage) : UIElement(stage) {}

void UIElementActivateble::activate() {
    m_isActivated = true;
    m_stage->activateUIElement(m_id);
}

void UIElementActivateble::deactivate() {
    m_isActivated = false;
    m_stage->deactivateUIElement(m_id);
}

void UIElementActivateble::onClick(int x, int y) {
    if (m_isActivated) {
        deactivate();
    } else {
        activate();
    }
}

void UIElementActivateble::onHotkey() {
    if (m_isActivated) {
        deactivate();
    } else {
        activate();
    }
}
