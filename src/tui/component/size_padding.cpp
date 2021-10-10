//
// Created by main on 29.12.2020.
//
#include <msg_terminal.hpp>

void SizePaddingComponent::setPadding(uint16_t padding) { _padding = padding; }
void SizePaddingComponent::setHeightWithPadding(uint16_t heightWithPadding) { _heightWithPadding = heightWithPadding; }
void SizePaddingComponent::setWidthWithPadding(uint16_t widthWithPadding) { _widthWithPadding = widthWithPadding; }

uint16_t SizePaddingComponent::getPadding() { return _padding; }
uint16_t SizePaddingComponent::getHeightWithPadding() { return _heightWithPadding; }
uint16_t SizePaddingComponent::getWidthWithPadding() { return  _widthWithPadding; }