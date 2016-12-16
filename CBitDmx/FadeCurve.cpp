//
//  FadeCurve.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/16/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "FadeCurve.hpp"

double calculateFadeCurve(FadeCurve fadeCurve, double time) {
    switch (fadeCurve) {
        case FadeCurve::linear:
            return time;
        case FadeCurve::squared:
            return time * time;
        case FadeCurve::cubed:
            return time * time * time;
    }
}


std::string getFadeCurveName(FadeCurve fadeCurve) {
    switch (fadeCurve) {
        case FadeCurve::linear:
            return "Linear";
        case FadeCurve::squared:
            return "Squared";
        case FadeCurve::cubed:
            return "Cubed";
    }
}
