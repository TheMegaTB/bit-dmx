//
//  Button.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Button_hpp
#define Button_hpp

#include "UIPart.hpp"


class Button : public UIPart {
public:
    Button(std::function<void(bool)> changeCallback, std::string caption, int width, int height, sf::Font font);
    
    void setPressed(bool pressed);
    void setCaption(std::string caption);
    
    virtual void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
    virtual void onMouseRelease(int x, int y, sf::Mouse::Button mouseButton);
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
protected:
    bool m_pressed;
    std::string m_caption;
    sf::Font m_font;
    std::function<void(bool)> m_changeCallback;
};

#endif /* Button_hpp */
