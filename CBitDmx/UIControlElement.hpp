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

#include "UIController.hpp"
#include "Button.hpp"
#include "Toggle.hpp"
#include "Slider.hpp"

#include "Stage.hpp"
#include "FadeCurve.hpp"

class UIControlElement : public UIController {
public:
    UIControlElement(Stage* stage, std::vector<std::shared_ptr<UIPart>> uiParts);
    
    virtual int getHeight() const;
    
    sf::Keyboard::Key getHotkey();
    void setHotkey(sf::Keyboard::Key hotkey);
    
    void setID(int id);
    void setFadeTime(sf::Time fadeTime);
    void setFadeCurve(FadeCurve fadeCurve);
    
    virtual void hotkeyWrapper(sf::Keyboard::Key hotkey);
    
    virtual void activate();
    virtual void deactivate();
    virtual void action();
    virtual void onHotkey();
    
    int m_id;
protected:
    bool m_isActivated;
    
    sf::Keyboard::Key m_hotkey;
    sf::Time m_fadeTime;
    FadeCurve m_fadeCurve;
    
    
    Stage *m_stage;
};

#endif /* UIElement_hpp */
