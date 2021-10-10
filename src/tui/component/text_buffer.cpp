#include <msg_terminal.hpp>

void BasicBufferComponent::setBuffer(wchar_t* buffer) {
  // wmemset(_buffer, '\0', wcslen(_buffer));
  delete [] _buffer;
  _buffer = buffer;

  // _buffer = new wchar_t[wcslen(buffer + 1)];
  // wcsncpy(_buffer, buffer, wcslen(buffer + 1));
  // _buffer = buffer;
}

void BasicBufferComponent::setCursor(uint16_t cursor) { _cursor = cursor; }

wchar_t* BasicBufferComponent::getBuffer() { return _buffer; }

uint16_t BasicBufferComponent::getCursor() { return _cursor; }
