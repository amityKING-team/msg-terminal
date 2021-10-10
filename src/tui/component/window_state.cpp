//
// Created by main on 05.01.2021.
//
#include <msh_terminal.hpp>

template<class Box>
void WindowStateComponent::add(Box box) {
  _stateBox.push_back(box);
}

template<class Box>
Box WindowStateComponent::next() {
  if(_stateIterator == _stateBox.end()) {
    _stateIterator = _stateBox.begin();
    return *_stateIterator;
  } else {
    _stateIterator++;
    return *_stateIterator;
  }
}