//
//  Button.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Button_hpp
#define Button_hpp

#include "Element.hpp"


class Button : public Element {
public:
    Button(std::string caption, int width, int height, sf::Font font);
    
    void onClick(std::function<void(bool)> clickCallback) {m_clickCallback = clickCallback;};
    
    void setPressed(bool pressed) { m_pressed = pressed; };
    void setCaption(std::string caption) { m_caption = caption; };
    void setColorActivated(sf::Color color) { m_colorActivated = color; };
    void setColorDeactivated(sf::Color color) { m_colorDeactivated = color; };
    
    virtual void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
    virtual void onMouseRelease(int x, int y, sf::Mouse::Button mouseButton);
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
protected:
    virtual bool drawActivated() const { return m_pressed; };
    
    bool m_pressed;
    std::string m_caption;
    
    sf::Color m_colorActivated;
    sf::Color m_colorDeactivated;
    sf::Font m_font;
    
    std::function<void(bool)> m_clickCallback;
};

#endif /* Button_hpp */
