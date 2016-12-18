//
//  Fixture.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "Fixture.hpp"


Fixture::Fixture(Stage *stage, std::string name, std::vector<int> channelGroups) {
    m_stage = stage;
    m_name = name;
    m_channelGroups = channelGroups;
}

std::vector<int> Fixture::getChannelGroups() {
    return m_channelGroups;
}


std::string Fixture::getName() {
    return m_name;
}
