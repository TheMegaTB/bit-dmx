//
//  Toggle.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Toggle_hpp
#define Toggle_hpp

#include "Button.hpp"

class Toggle : public Button {
public:
    Toggle(std::string caption, int width, int height, sf::Font font);
    
    void onChange(std::function<void(bool)> changeCallback) {m_changeCallback = changeCallback;};
    void setActivation(bool activated) { m_activated = activated; };
    
    virtual void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
protected:
    virtual bool drawActivated() const { return m_activated; };
    
    bool m_activated;
    std::function<void(bool)> m_changeCallback;
};

#endif /* Toggle_hpp */
