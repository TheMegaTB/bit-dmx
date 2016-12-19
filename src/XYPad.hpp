//
//  XYPad.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef XYPad_hpp
#define XYPad_hpp

#include "UIPart.hpp"

class XYPad : public UIPart {
public:
    XYPad(int minValue, int maxValue, std::function<void(double, double)> valueChangeCallback, std::function<void()> disableCallback, int width, int height, sf::Font font);
    
    void setRawValue(double xValue, double yValue, bool callback = true);
    int getXValue() const;
    int getYValue() const;
    
    virtual void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
    virtual void onMouseMove(int x, int y, sf::Mouse::Button mouseButton);
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
protected:
    
    double m_minValue;
    double m_maxValue;
    double m_xValue;
    double m_yValue;
    sf::Font m_font;
    std::function<void(double, double)> m_valueChangeCallback;
    std::function<void()> m_disableCallback;
};

#endif /* XYPad_hpp */
