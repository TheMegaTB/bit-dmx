//
//  ActionGroup.cpp
//  BitDmx
//
//  Created by Noah Peeters on 12/23/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "ActionGroup.hpp"


ActionTarget::ActionTarget(ActionType actionType, int address, std::string basename, std::string subname) {
    this->actionType = actionType;
    this->address = address;
    this->basename = basename;
    this->subname = subname;
}

ValuedActionGroup::ValuedActionGroup(Stage* stage) {
    m_stage = stage;
    m_buffered = false;
    m_id = m_stage->getNextID();
}

ValuedActionGroup::ValuedActionGroup(Stage* stage, json jsonData) : ValuedActionGroup(stage) {
    if (jsonData.is_object()) {
        for (json::iterator it = jsonData.begin(); it != jsonData.end(); ++it) {
            std::string basename = it.key();
            if (it.value().is_object()) {
                if (basename.find(":") != -1) {
                    add(basename, ValuedAction(it.value()));
                } else {
                    for (json::iterator it2 = it.value().begin(); it2 != it.value().end(); ++it2) {
                        std::string name = basename + ":" + it2.key();
                        if (it2.value().is_object()) {
                            add(name, ValuedAction(it2.value()));
                        } else {
                            std::cout << "Cannot add " << name << " to ValuedActionGroup without valid data:" << std::endl;
                            std::cout << std::setw(4) << jsonData << std::endl;
                            exit(1);
                        }
                    }
                }
            } else {
                std::cout << "Cannot add " << basename << " to ValuedActionGroup without an object:" << std::endl;
                std::cout << std::setw(4) << jsonData << std::endl;
                exit(1);
            }
        }
    } else {
        std::cout << "Cannot create ValuedActionGroup without an object:" << std::endl;
        std::cout << std::setw(4) << jsonData << std::endl;
        exit(1);
    }
}

void ValuedActionGroup::add(std::string id, ValuedAction action) {
    m_ids.push_back(std::make_pair(id, action));
    m_buffered = false;
}


std::vector<std::pair<ActionTarget, ValuedAction>> ValuedActionGroup::get() {
    if (m_buffered) {
        return m_idsBuffered;
    } else {
        m_idsBuffered.clear();
        m_idsBuffered.reserve(m_ids.size());
        
        for (int arrayID = 0; arrayID < m_ids.size(); arrayID++) {
            m_idsBuffered.push_back(std::make_pair(m_stage->stringIDToNumberID(m_ids[arrayID].first), m_ids[arrayID].second));
        }
        
        
        return m_idsBuffered;
    }
}

UnvaluedActionGroup::UnvaluedActionGroup(Stage* stage) {
    m_stage = stage;
    m_buffered = false;
    m_id = m_stage->getNextID();
}

UnvaluedActionGroup::UnvaluedActionGroup(Stage* stage, json jsonData) : UnvaluedActionGroup(stage) {
    if (jsonData.is_object()) {
        for (json::iterator it = jsonData.begin(); it != jsonData.end(); ++it) {
            std::string basename = it.key();
            if (it.value().is_object()) {
                if (basename.find(":") != -1) {
                    add(basename, UnvaluedAction(it.value()));
                } else {
                    for (json::iterator it2 = it.value().begin(); it2 != it.value().end(); ++it2) {
                        std::string name = basename + ":" + it2.key();
                        if (it2.value().is_object()) {
                            add(name, UnvaluedAction(it2.value()));
                        } else {
                            std::cout << "Cannot add " << name << " to UnvaluedAction without valid data:" << std::endl;
                            std::cout << std::setw(4) << jsonData << std::endl;
                            exit(1);
                        }
                    }
                }
            } else {
                std::cout << "Cannot add " << basename << " to UnvaluedAction without an object:" << std::endl;
                std::cout << std::setw(4) << jsonData << std::endl;
                exit(1);
            }
        }
    } else {
        std::cout << "Cannot create UnvaluedAction without an object:" << std::endl;
        std::cout << std::setw(4) << jsonData << std::endl;
        exit(1);
    }
}

void UnvaluedActionGroup::add(std::string id, UnvaluedAction action) {
    m_ids.push_back(std::make_pair(id, action));
    m_buffered = false;
}


std::vector<std::pair<ActionTarget, UnvaluedAction>> UnvaluedActionGroup::get() {
    if (m_buffered) {
        return m_idsBuffered;
    } else {
        m_idsBuffered.clear();
        m_idsBuffered.reserve(m_ids.size());
        
        for (int arrayID = 0; arrayID < m_ids.size(); arrayID++) {
            m_idsBuffered.push_back(std::make_pair(m_stage->stringIDToNumberID(m_ids[arrayID].first), m_ids[arrayID].second));
        }
        
        
        return m_idsBuffered;
    }
}
