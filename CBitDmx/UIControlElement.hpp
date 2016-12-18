//
//  UIElement.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIElement_hpp
#define UIElement_hpp

#include <stdio.h>

class UIControlElement;

#include <memory>

#include "UIController.hpp"
#include "Button.hpp"
#include "Toggle.hpp"
#include "Slider.hpp"
#include "XYPad.hpp"
#include "Selector.hpp"


#include "Stage.hpp"
#include "FadeCurve.hpp"


enum UIControlElementType {
    UIControlElementSwitch = 0,
    UIControlElementPushButton,
    UIControlElementChannel,
    UIControlElementXYPad
};

class UIControlElement : public UIController {
public:
    UIControlElement(Stage* stage, int width, int height);
    
    void setID(int id);
    void setFadeTime(sf::Time fadeTime);
    void setFadeCurve(FadeCurve fadeCurve);
    
    virtual void hotkeyWrapper(sf::Keyboard::Key hotkey) {};
    virtual void hotkeyReleaseWrapper(sf::Keyboard::Key hotkey) {};
    
    virtual void activate();
    virtual void deactivate();
    virtual void action() {};
    
    void drawEditor(sf::RenderTarget& target, sf::RenderStates states) const;
    virtual void drawSubEditor(sf::RenderTarget& target, sf::RenderStates states) const {};
    
    int m_id;
protected:
    bool m_isActivated;
    
    sf::Time m_fadeTime;
    FadeCurve m_fadeCurve;
    
    Stage *m_stage;
};

#endif /* UIElement_hpp */
