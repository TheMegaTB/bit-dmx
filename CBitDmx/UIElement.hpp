//
//  UIElement.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIElement_hpp
#define UIElement_hpp

class UIElement;

#include <stdio.h>

#include <SFML/Graphics/Drawable.hpp>
#include <SFML/Graphics/Transformable.hpp>
#include <SFML/Graphics.hpp>

#include "Stage.hpp"
#include "FadeCurve.hpp"

class UIElement : public sf::Drawable, public sf::Transformable {
public:
    UIElement(Stage* stage);
    virtual int getHeight() const;
    
    void setID(int id);
    void setFadeTime(sf::Time fadeTime);
    void setFadeCurve(FadeCurve fadeCurve);
    
    virtual void action();
    virtual void onClick(int x, int y);
    virtual void onHotkey();
protected:
    sf::Time m_fadeTime;
    FadeCurve m_fadeCurve;
    
    int m_width;
    int m_id;
    Stage *m_stage;
    
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
};

#endif /* UIElement_hpp */
