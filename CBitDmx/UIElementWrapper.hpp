//
//  UIElementWrapper.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIElementWrapper_hpp
#define UIElementWrapper_hpp

class UIElementWrapper;

#include <stdio.h>
#include <SFML/Graphics.hpp>

#include "Stage.hpp"

class UIElementWrapper {
public:
    UIElementWrapper(std::shared_ptr<UIElement> uiElement);
    sf::Keyboard::Key getHotkey();
    void setHotkey(sf::Keyboard::Key hotkey);
    
    std::shared_ptr<UIElement> uiElement;
    
    void onClick(int x, int y);
    void onHotkey(sf::Keyboard::Key hotkey);
private:
    sf::Keyboard::Key m_hotkey;
};

#endif /* UIElementWrapper_hpp */
