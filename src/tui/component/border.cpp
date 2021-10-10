#include <msg_terminal.hpp>

void BorderComponent::setHaveBorder(bool haveBorder) {
  _haveBorder = haveBorder;
}

bool BorderComponent::getHaveBorder() { return _haveBorder; }
