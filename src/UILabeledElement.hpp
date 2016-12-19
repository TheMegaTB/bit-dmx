//
//  UILabeledElement.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UILabeledElement_hpp
#define UILabeledElement_hpp

#include <stdio.h>

#include "UIControlElement.hpp"

class UILabeledElement : public UIControlElement {
public:
    UILabeledElement(Stage* stage, int width, int height);
    
    virtual void setCaption(std::string caption);
    virtual void addPart(std::shared_ptr<UIPart> part);
private:
    std::shared_ptr<Label> m_label;
};


#endif /* UILabeledElement_hpp */
