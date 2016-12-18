//
//  UIChaser.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIChaser_hpp
#define UIChaser_hpp

#include <stdio.h>

#include "UILabeledElement.hpp"

class UIChaser : public UILabeledElement {
public:
    UIChaser(Stage* stage, json chaserData);
    
    
    virtual void update();
    
    virtual void chaserActivate();
    virtual void chaserDeactivate();
    
    virtual void activate();
    virtual void deactivate();
private:
    std::shared_ptr<Toggle> m_toggle;
    
    sf::Time m_startTime;
    
    int m_position;
    std::vector<json> m_chaserData;
};

#endif /* UIChaser_hpp */
