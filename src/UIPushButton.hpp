//
//  UIPushButton.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIPushButton_hpp
#define UIPushButton_hpp

#include <stdio.h>

#include "UISingleVChannel.hpp"

class UIPushButton : public UISingleVChannel {
public:
    UIPushButton(Stage* stage, ValuedActionGroup actionGroup);
    
    void setCaption(std::string caption);
    
    virtual void update();
    
    virtual void onHotkey();
    virtual void onHotkeyRelease();
private:
    std::shared_ptr<Button> m_button;
    ValuedActionGroup m_actionGroup;
};

#endif /* UIPushButton_hpp */
