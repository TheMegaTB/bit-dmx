//
//  Switch.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UISwitch_hpp
#define UISwitch_hpp

#include <stdio.h>

#include "UISingleVChannel.hpp"

class UISwitch : public UISingleVChannel {
public:
    UISwitch(Stage* stage, ValuedActionGroup actionGroup);
    
    void setCaption(std::string caption);
    virtual void update();
private:
    std::shared_ptr<Toggle> m_toggle;
    ValuedActionGroup m_actionGroup;
};


#endif /* UISwitch_hpp */
