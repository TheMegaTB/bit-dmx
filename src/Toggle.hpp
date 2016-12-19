//
//  Toggle.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/17/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Toggle_hpp
#define Toggle_hpp

#include "UIPart.hpp"

class Toggle : public UIPart {
public:
    Toggle(std::function<void(bool)> clickCallback, std::string caption, int width, int height, sf::Font font);
    
    void setActivation(bool activated);
    void setCaption(std::string caption);
    
    virtual void onMousePress(int x, int y, sf::Mouse::Button mouseButton);
    virtual void draw(sf::RenderTarget& target, sf::RenderStates states) const;
protected:
    bool m_activated;
    std::string m_caption;
    sf::Font m_font;
    std::function<void(bool)> m_clickCallback;
};

#endif /* Toggle_hpp */
