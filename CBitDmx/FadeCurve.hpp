//
//  FadeCurve.hpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#ifndef FadeCurve_hpp
#define FadeCurve_hpp

#include <stdio.h>
#include <string>
#include <vector>

enum FadeCurve {
    linear = 0,
    squared,
    cubed
};

#define fadeCurveEnumSize 3

double calculateFadeCurve(FadeCurve fadeCurve, double time);
std::string getFadeCurveName(FadeCurve fadeCurve);



#endif /* FadeCurve_hpp */
