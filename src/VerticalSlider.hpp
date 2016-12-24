//
//  VerticalSlider.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Slider_hpp
#define Slider_hpp

#include "Element.hpp"

class VerticalSlider : public Element {
public:
    VerticalSlider(int minValue, int maxValue, int width, int height, sf::Font font);
    
    void onChange(std::function<void(bool)> changeCallback) {m_changeCallback = changeCallback;};
    void onDisable(std::function<void()> disableCallback) {m_disableCallback = disableCallback;};
    
    int getValue() const { return round(m_minValue + (m_maxValue - m_minValue) * m_value); };
    void setValue(int value) { m_value = (value - m_minValue)/(m_maxValue - m_minValue); };
    
    virtual void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
    virtual void onMouseDrag(int x, int y, sf::Mouse::Button mouseButton);
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
protected:
    
    void setRawValue(double value);
    
    double m_minValue;
    double m_maxValue;
    double m_value;
    sf::Font m_font;
    std::function<void(double)> m_changeCallback;
    std::function<void()> m_disableCallback;
};

#endif /* Slider_hpp */
