//
//  XYPad.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef XYPad_hpp
#define XYPad_hpp

#include "Element.hpp"

class XYPad : public Element {
public:
    XYPad(int minValue, int maxValue, int width, int height, sf::Font font);
    
    void onChange(std::function<void(double, double)> changeCallback) {m_changeCallback = changeCallback;};
    void onDisable(std::function<void()> disableCallback) {m_disableCallback = disableCallback;};
    
    void setRawValue(double xValue, double yValue);
    void setValue(double xValue, double yValue);
    void setXValue(double xValue);
    void setYValue(double yValue);
    int getXValue() const;
    int getYValue() const;
    
    virtual void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
    virtual void onMouseDrag(int x, int y, sf::Mouse::Button mouseButton);
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
protected:
    
    double m_minValue;
    double m_maxValue;
    double m_xValue;
    double m_yValue;
    sf::Font m_font;
    std::function<void(double, double)> m_changeCallback;
    std::function<void()> m_disableCallback;
};

#endif /* XYPad_hpp */
