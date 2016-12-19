//
//  UISingleHotkey.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UISingleHotkey_hpp
#define UISingleHotkey_hpp

#include <stdio.h>

#include "UIControlElement.hpp"

class UISingleHotkey : public UIControlElement {
public:
    UISingleHotkey(Stage* stage, int width, int height, sf::Keyboard::Key hotkey = sf::Keyboard::Unknown);
    
    sf::Keyboard::Key getHotkey();
    
    void setHotkey(sf::Keyboard::Key hotkey);
    
    virtual void hotkeyWrapper(sf::Keyboard::Key hotkey);
    virtual void hotkeyReleaseWrapper(sf::Keyboard::Key hotkey);
    virtual void onHotkey();
    virtual void onHotkeyRelease() {};
private:
    sf::Keyboard::Key m_hotkey;
};

#endif /* UISingleHotkey_hpp */
