//
// Created by main on 06.01.2021.
//
#include <msg_terminal.hpp>

void PositionPaddingComponent::setPositionWithPaddingY(
    uint16_t positionWithPadding_y) {
  _positionWithPadding_y = positionWithPadding_y;
}

void PositionPaddingComponent::setPositionWithPaddingX(
    uint16_t positionWithPadding_x) {
  _positionWithPadding_x = positionWithPadding_x;
}

uint16_t PositionPaddingComponent::getPositionWithPaddingY() {
  return _positionWithPadding_y;
}
uint16_t PositionPaddingComponent::getPositionWithPaddingX() {
  return _positionWithPadding_x;
}