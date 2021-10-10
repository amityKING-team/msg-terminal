//
// Created by main on 05.01.2021.
//
#include <msg_terminal.hpp>

void TerminalStateComponent::setState(uint16_t state) {
  _state = state;
}

uint16_t TerminalStateComponent::getState() { return _state; }