//
// Created by main on 05.01.2021.
//
#include <msg_terminal.hpp>

void InputKeyboardComponent::setInputKey(uint16_t inputKey) {
  _inputKey = inputKey;
}

uint16_t InputKeyboardComponent::getInputKey() { return _inputKey; }