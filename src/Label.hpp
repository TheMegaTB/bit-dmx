//
//  Label.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Label_hpp
#define Label_hpp

#include "Element.hpp"


class Label : public Element {
public:
    Label(std::string caption, int width, int height, sf::Font font);
    
    void setCaption(std::string caption);
    
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
protected:
    std::string m_caption;
    sf::Font m_font;
};

#endif /* Label_hpp */
