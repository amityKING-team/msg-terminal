#include <msg_terminal.hpp>

TextBoxEntity::TextBoxEntity(
    SizeComponent sizeComponent, AreaComponent areaComponent,
    PositionComponent positionComponent,
    BasicBufferComponent basicBufferComponent, BorderComponent borderComponent,
    SizePaddingComponent sizePaddingComponent,
    AreaPaddingComponent areaPaddingComponent,
    PositionPaddingComponent positionPaddingComponent) {
  _sizeComponent = sizeComponent;
  _areaComponent = areaComponent;
  _positionComponent = positionComponent;
  _basicBufferComponent = basicBufferComponent;
  _borderComponent = borderComponent;
  _sizePaddingComponent = sizePaddingComponent;
  _areaPaddingComponent = areaPaddingComponent;
  _positionPaddingComponent = positionPaddingComponent;
}

/*TextBoxEntity::~TextBoxEntity() {
 delete [] _sizeComponent;
 delete [] _areaComponent;
 delete [] _positionComponent;
 delete [] _bufferComponent;
 delete [] _borderComponent;
}*/

SizeComponent& TextBoxEntity::getSizeComponent() { return _sizeComponent; }

AreaComponent& TextBoxEntity::getAreaComponent() { return _areaComponent; }

PositionComponent& TextBoxEntity::getPositionComponent() {
  return _positionComponent;
}

BasicBufferComponent& TextBoxEntity::getBasicBufferComponent() {
  return _basicBufferComponent;
}

BorderComponent& TextBoxEntity::getBorderComponent() {
  return _borderComponent;
}

SizePaddingComponent& TextBoxEntity::getSizePaddingComponent() {
  return _sizePaddingComponent;
}

AreaPaddingComponent& TextBoxEntity::getAreaPaddingComponent() {
  return _areaPaddingComponent;
}

PositionPaddingComponent& TextBoxEntity::getPositionPaddingComponent() {
  return _positionPaddingComponent;
}