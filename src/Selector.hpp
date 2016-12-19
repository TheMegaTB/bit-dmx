//
//  Selector.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Selector_hpp
#define Selector_hpp

#include <stdio.h>
#include "UIController.hpp"
#include "Button.hpp"
#include "Label.hpp"

class Selector : public UIController {
public:
    Selector(std::function<void(bool)> changeCallback, std::vector<std::string> options, int width, int height, sf::Font font);
    
    void select(int position);
    void next();
    void previous();
private:
    int m_currentPosition;
    std::vector<std::string> m_options;
    std::function<void(bool)> m_changeCallback;
    std::shared_ptr<Button> m_buttonLeft;
    std::shared_ptr<Button> m_buttonRight;
    std::shared_ptr<Label> m_label;
};


#endif /* Selector_hpp */
