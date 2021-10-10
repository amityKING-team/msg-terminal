#include <msg_terminal.hpp>

void SizeComponent::setHeight(uint16_t height) { _height = height; }

void SizeComponent::setWidth(uint16_t width) { _width = width; }

void SizeComponent::setDimension(std::vector<uint16_t> dimension) {
  _dimension = &dimension;
}

uint16_t SizeComponent::getHeight() { return _height; }

uint16_t SizeComponent::getWidth() { return _width; }

std::vector<uint16_t> SizeComponent::getDimension() { return *_dimension; }
