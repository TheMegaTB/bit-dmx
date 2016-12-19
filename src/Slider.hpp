//
//  Slider.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Slider_hpp
#define Slider_hpp

#include "UIPart.hpp"

class Slider : public UIPart {
public:
    Slider(int minValue, int maxValue, std::function<void(double)> valueChangeCallback, std::function<void()> disableCallback, int width, int height, sf::Font font);
    
    void setRawValue(double value, bool callback = true);
    int getValue() const;
    
    virtual void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
    virtual void onMouseMove(int x, int y, sf::Mouse::Button mouseButton);
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
protected:
    
    double m_minValue;
    double m_maxValue;
    double m_value;
    sf::Font m_font;
    std::function<void(double)> m_valueChangeCallback;
    std::function<void()> m_disableCallback;
};

#endif /* Slider_hpp */
