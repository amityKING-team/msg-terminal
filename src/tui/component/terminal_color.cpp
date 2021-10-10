#include <msg_terminal.hpp>

void TerminalColorComponent::setHowColor(uint16_t howColor) {
  _howColor = howColor;
}
void TerminalColorComponent::setTerminalEnvVariable(
    wchar_t* terminalEnvVariable) {
  delete[] _terminalEnvVariable;
  _terminalEnvVariable = terminalEnvVariable;
}

uint16_t TerminalColorComponent::getHowColor() { return _howColor; }
wchar_t* TerminalColorComponent::getTerminalEnvVariable() {
  return _terminalEnvVariable;
}
