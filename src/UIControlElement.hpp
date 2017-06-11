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

#include "ElementController.hpp"
#include "Button.hpp"
#include "Toggle.hpp"
#include "HorizontalSlider.hpp"
#include "VerticalSlider.hpp"
#include "XYPad.hpp"


#include "Stage.hpp"
#include "FadeCurve.hpp"


enum UIControlElementType {
    UIControlElementChaser = 0,
    UIControlElementSwitch,
    UIControlElementPushButton,
    UIControlElementSlieder,
    UIControlElementXYPad
};

class UIControlElement : public ElementController {
public:
    UIControlElement(Stage* stage, int width, int height);
    
    virtual void setValue(std::string subname, ChannelValue value, int activationID) {};
    virtual void startFade(std::string subname, sf::Time fadeTime, ChannelValue value, FadeCurve fadeCurve, int activationID) {};
    virtual void deactivateActivation(int activationID) {};
    virtual bool isActivated() { return false; };
    
    virtual void update() {};
    
    
    // caption & hotkey
    sf::Keyboard::Key getHotkey() { return m_hotkey; };
    void setHotkey(sf::Keyboard::Key hotkey) { m_hotkey = hotkey; };
    virtual void setCaption(std::string caption) { m_caption = caption; };
    
    virtual void hotkeyWrapper(sf::Keyboard::Key hotkey);
    virtual void hotkeyReleaseWrapper(sf::Keyboard::Key hotkey);
    virtual void onHotkey() {};
    virtual void onHotkeyRelease() {};
protected:
    sf::Keyboard::Key m_hotkey;
    std::string m_caption;
    
    Stage *m_stage;
};

#endif /* UIElement_hpp */
