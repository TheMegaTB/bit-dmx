//
//  UIController.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIController_hpp
#define UIController_hpp

#include <memory>

#include "UIPart.hpp"

class UIController : public UIPart {
public:
    UIController(std::vector<std::shared_ptr<UIPart>> uiParts);
    virtual void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
    virtual void onMouseMove(int x, int y, sf::Mouse::Button mouseButton);
    virtual void onMouseRelease(int x, int y, sf::Mouse::Button mouseButton);
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
protected:
    int m_lastClickOn;
    std::vector<std::shared_ptr<UIPart>> m_uiParts;
};

#endif /* UIController_hpp */
