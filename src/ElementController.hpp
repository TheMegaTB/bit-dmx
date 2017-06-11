//
//  UIController.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIController_hpp
#define UIController_hpp

#include <memory>

#include "Element.hpp"

class ElementController : public Element {
public:
    ElementController(int width, int height) : Element(width, height) {};
    ElementController(std::vector<std::shared_ptr<Element>> elements, int width, int height);
    
    virtual void clear() { m_elements.clear(); };
    virtual void addElement(std::shared_ptr<Element> element);
    
    virtual void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
    virtual void onMouseDrag(int x, int y, sf::Mouse::Button mouseButton);
    virtual void onMouseRelease(int x, int y, sf::Mouse::Button mouseButton);
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
protected:
    
    int findElementByXY(int x, int y);
    int m_lastClickOn;
private:
    std::vector<std::shared_ptr<Element>> m_elements;
};


#endif /* UIController_hpp */
