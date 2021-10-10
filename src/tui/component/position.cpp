#include <msg_terminal.hpp>

void PositionComponent::setPositionY(uint16_t position_y) {
  _position_y = position_y;
}

void PositionComponent::setPositionX(uint16_t position_x) {
  _position_x = position_x;
}

void PositionComponent::setDimensionPosition(
    std::vector<uint16_t> dimensionPosition) {
  _dimensionPosition = &dimensionPosition;
}

uint16_t PositionComponent::getPositionY() { return _position_y; }

uint16_t PositionComponent::getPositionX() { return _position_x; }

std::vector<uint16_t> PositionComponent::getDimensionPosition() {
  return *_dimensionPosition;
}
