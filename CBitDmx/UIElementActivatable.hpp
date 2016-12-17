//
//  UIElementActivatable.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIElementActivatable_hpp
#define UIElementActivatable_hpp

#include <stdio.h>

#include "UIElement.hpp"

class UIElementActivateble : public UIElement {
public:
    UIElementActivateble(Stage* stage);
    virtual void activate();
    virtual void deactivate();
    virtual void onClick(int x, int y);
    virtual void onHotkey();
protected:
    bool m_isActivated;
};

#endif /* UIElementActivatable_hpp */
