//
//  UIChaser.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef UIChaser_hpp
#define UIChaser_hpp

#include <stdio.h>

#include "UISingleVChannel.hpp"

class ChaserStep {
public:
    ChaserStep(Stage *stage, json jsonData);
    
    ValuedActionGroup getActionGroup() { return m_actionGroup; }
    sf::Time getChaserTime() const { return m_chaserTime; };
    
    bool activate() { return m_activate; };
    bool deactivate() { return m_deactivate; };
    
    bool inRound(int round);
    
private:
    ValuedActionGroup m_actionGroup;
    
    sf::Time m_chaserTime;
    
    int m_minRound;
    int m_round;
    int m_maxRound;
    
    bool m_activate;
    bool m_deactivate;
};


class UIChaser : public UISingleVChannel {
public:
    UIChaser(Stage* stage, std::vector<ChaserStep> chaserSteps);
    virtual void update();
private:
    std::shared_ptr<Toggle> m_toggle;
    
    sf::Time m_startTime;
    
    void next();
    void activateStep(int id);
    void deactivateStep(int id);
    
    int m_position;
    int m_round;
    std::vector<ChaserStep> m_chaserSteps;
};

#endif /* UIChaser_hpp */
