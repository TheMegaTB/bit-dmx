//
//  UISingleHotkey.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UISingleHotkey.hpp"


UISingleHotkey::UISingleHotkey(Stage* stage, int width, int height, sf::Keyboard::Key hotkey) : UIControlElement(stage, width, height) {
    m_hotkey = hotkey;
}

sf::Keyboard::Key UISingleHotkey::getHotkey() {
    return m_hotkey;
}

void UISingleHotkey::setHotkey(sf::Keyboard::Key hotkey) {
    m_hotkey = hotkey;
}

void UISingleHotkey::hotkeyWrapper(sf::Keyboard::Key hotkey) {
    if (m_hotkey == hotkey) {
        onHotkey();
    }
}

void UISingleHotkey::hotkeyReleaseWrapper(sf::Keyboard::Key hotkey) {
    if (m_hotkey == hotkey) {
        onHotkeyRelease();
    }
}

void UISingleHotkey::onHotkey() {
    if (m_isActivated) {
        deactivate();
    } else {
        activate();
    }
}
