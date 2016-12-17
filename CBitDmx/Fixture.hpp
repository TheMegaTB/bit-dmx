//
//  Fixture.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef Fixture_hpp
#define Fixture_hpp

class Fixture;

#include <stdio.h>

#include "Stage.hpp"

class Fixture {
public:
    Fixture(Stage *stage, std::string name, std::vector<int> channelGroups);
    std::vector<int> getChannelGroups();
private:
    Stage * m_stage;
    std::string m_name;
    std::vector<int> m_channelGroups;
};

#endif /* Fixture_hpp */
