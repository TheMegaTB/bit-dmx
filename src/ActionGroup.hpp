//
//  ActionGroup.hpp
//  BitDmx
//
//  Created by Noah Peeters on 12/23/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef ActionGroup_hpp
#define ActionGroup_hpp

class ValuedActionGroup;
class UnvaluedActionGroup;
class ActionTarget;


#include <stdio.h>
#include <vector>

#include "Action.hpp"
#include "Stage.hpp"

class ActionTarget {
public:
    ActionTarget(ActionType actionType, int address, std::string basename, std::string subname);
    ActionType actionType;
    int address;
    std::string basename;
    std::string subname;
};

class ValuedActionGroup {
public:
    ValuedActionGroup(Stage* stage);
    ValuedActionGroup(Stage* stage, json jsonData);
    
    int getID() const { return m_id; };
    
    void add(std::string id, ValuedAction action);
    std::vector<std::pair<ActionTarget, ValuedAction>> get();
private:
    std::vector<std::pair<std::string, ValuedAction>> m_ids;
    std::vector<std::pair<ActionTarget, ValuedAction>> m_idsBuffered;
    bool m_buffered;
    Stage* m_stage;
    int m_id;
};

class UnvaluedActionGroup {
public:
    UnvaluedActionGroup(Stage* stage);
    UnvaluedActionGroup(Stage* stage, json jsonData);
    
    int getID() const { return m_id; };
    
    void add(std::string id, UnvaluedAction action);
    std::vector<std::pair<ActionTarget, UnvaluedAction>> get();
private:
    std::vector<std::pair<std::string, UnvaluedAction>> m_ids;
    std::vector<std::pair<ActionTarget, UnvaluedAction>> m_idsBuffered;
    bool m_buffered;
    Stage* m_stage;
    int m_id;
};

#endif /* ActionGroup_hpp */
