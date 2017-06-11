//
//  UIChannel.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UISlieder_hpp
#define UISlieder_hpp

#include <stdio.h>

#include "UISingleVChannel.hpp"

class UISlieder : public UISingleVChannel {
public:
    UISlieder(Stage* stage, UnvaluedActionGroup actionGroup);
    virtual void update();
    
private:
    std::shared_ptr<HorizontalSlider> m_slider;
    UnvaluedActionGroup m_actionGroup;
};


#endif /* UISlieder_hpp */
