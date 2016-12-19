//
//  UILabeledElement.cpp
//  CBitDmx
//
//  Created by Noah Peeters on 12/18/16.
//  Copyright © 2016 BitDmx. All rights reserved.
//

#include "UILabeledElement.hpp"

UILabeledElement::UILabeledElement(Stage* stage, int width, int height) : UIControlElement(stage, width, height + stage->UIPartWidth / 4) {
    m_label = std::make_shared<Label>("Unnamed", stage->UIPartWidth, stage->UIPartWidth / 4, m_stage->getFont());
    
    UIControlElement::addPart(m_label);
}


void UILabeledElement::setCaption(std::string caption) {
    UIControlElement::setCaption(caption);
    m_label->setCaption(caption);
}

void UILabeledElement::addPart(std::shared_ptr<UIPart> part) {
    part->move(0, m_stage->UIPartWidth / 4);
    UIController::addPart(part);
}
