//
//  UIPart.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIPart_hpp
#define UIPart_hpp

#include <vector>
#include <stdio.h>
#include <cmath>
#include <iostream>

#include "json.hpp"
using json = nlohmann::json;

#include <SFML/Graphics.hpp>

#include "Types.hpp"

class UIPart : public sf::Drawable, public sf::Transformable {
public:
    UIPart(int width, int height);
    virtual int getWidth() const { return m_width; };
    virtual int getHeight() const { return m_height; };
    
    virtual void onMousePress(int x, int y, sf::Mouse::Button mouseButton) {};
    virtual void onMouseMove(int x, int y, sf::Mouse::Button mouseButton) {};
    virtual void onMouseRelease(int x, int y, sf::Mouse::Button mouseButton) {};
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
private:
    int m_height;
    int m_width;
};

#endif /* UIPart_hpp */
