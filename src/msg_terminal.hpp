#pragma once

//#include <ncurses.h>
#include <ncursesw/ncurses.h>

#include <cwchar>
#include <deque>
#include <vector>

/*
 * `SizeComponent` is size of window in terminal.
 */
class SizeComponent {
 private:
  uint16_t _height;
  uint16_t _width;
  // Use this if size more that width and height. Like 3D, 4D – 5Head
  std::vector<uint16_t>* _dimension;

 public:
  void setHeight(uint16_t height);
  void setWidth(uint16_t width);
  void setDimension(std::vector<uint16_t> dimension);

  uint16_t getHeight();
  uint16_t getWidth();
  std::vector<uint16_t> getDimension();
};

/*
 * `AreaComponent` stores value of area of window.
 */
class AreaComponent {
 private:
  uint32_t _area;

 public:
  void setArea(uint32_t area);

  uint32_t getArea();
};

/*
 * `PositionComponent` stores position of window.
 */
class PositionComponent {
 private:
  uint16_t _position_y;
  uint16_t _position_x;
  std::vector<uint16_t>* _dimensionPosition;

 public:
  void setPositionY(uint16_t position_y);
  void setPositionX(uint16_t position_x);
  // Use if more dimension
  void setDimensionPosition(std::vector<uint16_t> dimensionPosition);

  uint16_t getPositionY();
  uint16_t getPositionX();
  std::vector<uint16_t> getDimensionPosition();
};

/*
 * I know that Ncurses have struct `WINDOW*` and `border()`. But I have decided
 * use own structures (classes).
 */
/*
 * TODO: Create left-top, right-top, ... corner
 */
class BorderComponent {
 private:
  bool _haveBorder;
  // ls, rs, ts, bs, tl, tr, bl, br
  std::vector<cchar_t> _symbolsBorder;
  wchar_t* _nameBorder;

 public:
  void setHaveBorder(bool haveBorder);
  void setBorder();

  bool getHaveBorder();

  std::vector<cchar_t> getBorder();
};

class SizePaddingComponent {
 private:
  uint16_t _padding;
  uint16_t _widthWithPadding;
  uint16_t _heightWithPadding;
  std::vector<uint16_t>* _dimension;

 public:
  void setPadding(uint16_t padding);
  void setHeightWithPadding(uint16_t widthWithPadding);
  void setWidthWithPadding(uint16_t heightWithPadding);

  uint16_t getPadding();
  uint16_t getHeightWithPadding();
  uint16_t getWidthWithPadding();
};

class PositionPaddingComponent {
 private:
  uint16_t _positionWithPadding_y;
  uint16_t _positionWithPadding_x;
  std::vector<uint16_t>* _dimensionWithPaddingPosition;

 public:
  void setPositionWithPaddingY(uint16_t positionWithPadding_y);
  void setPositionWithPaddingX(uint16_t positionWIthPadding_x);
  // Use if more dimension
  void setDimensionWithPaddingPosition(
      std::vector<uint16_t> dimensionWithPaddingPosition);

  uint16_t getPositionWithPaddingY();
  uint16_t getPositionWithPaddingX();
  std::vector<uint16_t> getDimensionWithPaddingPosition();
};

class AreaPaddingComponent {
 private:
  uint16_t _areaWithPadding;

 public:
  void setAreaWithPadding(uint16_t areaWithPadding);

  uint16_t getAreaWithPadding();
};

/*
 * `BufferComponent` stores all buffer of window.
 * The main reason why I use `wchar_t` instead of
 * `char*`, `std::string/std::wstring`, 'char16_t', 'tchar', `lpstr`,
 * 'sheet_w_char_k+d+b',
 * 'fuck_off_wchchar_string_stdnouseuft8butuseutf8_nowin32_t', etc.
 * is Ncurses library, coz Ncurses use cchar_t(struct with wchar_t and
 * attributes) type for color, animation.
 *
 * `_cursor` is position in buffer (to scrollable element in future).
 */
class BasicBufferComponent {
 private:
  wchar_t* _buffer;
  uint16_t _cursor;

 public:
  void setBuffer(wchar_t* buffer);
  void setCursor(uint16_t cursor);

  wchar_t* getBuffer();
  uint16_t getCursor();
};

/*
 * Default TextBox (like TextBox in WPF)
 */
class TextBoxEntity {
 private:
  SizeComponent _sizeComponent;
  AreaComponent _areaComponent;
  PositionComponent _positionComponent;
  BasicBufferComponent _basicBufferComponent;
  BorderComponent _borderComponent;
  SizePaddingComponent _sizePaddingComponent;
  AreaPaddingComponent _areaPaddingComponent;
  PositionPaddingComponent _positionPaddingComponent;

 public:
  SizeComponent& getSizeComponent();
  AreaComponent& getAreaComponent();
  PositionComponent& getPositionComponent();
  BasicBufferComponent& getBasicBufferComponent();
  BorderComponent& getBorderComponent();
  SizePaddingComponent& getSizePaddingComponent();
  AreaPaddingComponent& getAreaPaddingComponent();
  PositionPaddingComponent& getPositionPaddingComponent();

  void setSizeComponent();

  TextBoxEntity(SizeComponent sizeComponent, AreaComponent areaComponent,
                PositionComponent positionComponent,
                BasicBufferComponent bufferComponent,
                BorderComponent borderComponent,
                SizePaddingComponent sizePaddingComponent,
                AreaPaddingComponent areaPaddingComponent,
                PositionPaddingComponent positionPaddingComponent);
  // ~TextBoxEntity();
};

/*
 * Use Builder pattern: https://refactoring.guru/ru/design-patterns/builder !
 *
 * I don't like style first `height` then `width`. We just follow Nucrses style.
 *
 * At this moment I remembered Java:
 * IBaseEnterpriseVisitorOutputPrintableContextGeneratorSingletonMiddlewareStrategy.class
 * Source:
 * https://me.me/i/java-like-maybe-5-or-6-right-now-my-dude-cf5147b8b8c1461c836c04f71e51943a
 */
class TextBoxEntityBuilderSystem {
 private:
  SizeComponent _sizeComponent{};
  AreaComponent _areaComponent{};
  PositionComponent _positionComponent{};
  BasicBufferComponent _basicBufferComponent{};
  BorderComponent _borderComponent;
  SizePaddingComponent _sizePaddingComponent{};
  AreaPaddingComponent _areaPaddingComponent{};
  PositionPaddingComponent _positionPaddingComponent{};

 public:
  TextBoxEntityBuilderSystem& size(uint16_t height, uint16_t width);
  TextBoxEntityBuilderSystem& position(uint16_t y, uint16_t x);
  TextBoxEntityBuilderSystem& setBorder(bool border);
  TextBoxEntityBuilderSystem& buffer(wchar_t* buffer);

  TextBoxEntity build();

};

// Aphex Twin – Tha

/*
 * `ColorComponent` stores how color terminal use.
 * Terminator use 256, default Arch linux console use 8.
 */
class TerminalColorComponent {
 private:
  uint16_t _howColor;
  /*
   * _terminalEnvVariable storage "echo $TERM"(run in terminal) variable.
   * If use "xterm-256color" then terminal supports 256 colors.
   * Default console use "linux"(8 colors).
   */
  wchar_t* _terminalEnvVariable;

 public:
  void setHowColor(uint16_t _howColor);
  void setTerminalEnvVariable(wchar_t* terminalEnvVariable);

  uint16_t getHowColor();
  wchar_t* getTerminalEnvVariable();
};

/*
 * `TerminalEntity` stores size of terminal and color support
 */
class TerminalEntity {
 private:
  SizeComponent _sizeComponent{};
  AreaComponent _areaComponent{};
  TerminalColorComponent _terminalColorComponent{};

 public:
  void setSizeComponent(SizeComponent sizeComponent);
  void setAreaComponent(AreaComponent areaComponent);
  void setTerminalColorComponent(TerminalColorComponent terminalColorComponent);

  SizeComponent getSizeComponent();
  AreaComponent getAreaComponent();
  TerminalColorComponent getTerminalColorComponent();

  TerminalEntity(SizeComponent sizeComponent, AreaComponent areaComponent,
                 TerminalColorComponent colorComponent);
};

class TerminalEntityBuilderSystem {
 private:
  SizeComponent _sizeComponent;
  AreaComponent _areaComponent;
  TerminalColorComponent _terminalColorComponent;

  wchar_t* getWideString(const char* str);

 public:
  TerminalEntity build();
};

/*
 * `TextBoxPreparePrintTerminalSystem` prepare `TextBoxEntity` data to print in
 * terminal. This class consider user settings (in future)
 */

class TextBoxEntityPrepareBufferPrintTerminalSystem {
 private:
  std::deque<TextBoxEntity*> _containerTextBoxEntity;
  cchar_t _buffer;

 public:
  void add();
  void run();
  // void update();
};

class TextBoxEntityCalculateWithPaddingTerminalSystem {
 private:
  std::deque<TextBoxEntity*> _containerTextBoxEntity;

 public:
  void add(TextBoxEntity& textBoxEntity);
  void run();
};

/*
 *
 */
class TextBoxEntityPrintSystem {
 private:
  std::deque<TextBoxEntity*> _containerTextBoxEntity;
  TextBoxEntityPrepareBufferPrintTerminalSystem _textBoxEPBS;

 public:
  void add(TextBoxEntity& textBoxEntity);
  void run();
};

class InputKeyboardComponent {
 private:
  int _inputKey;

 public:
  void setInputKey(uint16_t inputKey);

  uint16_t getInputKey();
};

class InputKeyboardEntity {
 private:
  InputKeyboardComponent _inputKeyboardComponent;

 public:
  void setInputKeyboardComponent(InputKeyboardComponent inputKeyboardComponent);

  InputKeyboardComponent getInputKeyboardComponent();
};

class InputKeyboardEntitySystem {
 private:
  InputKeyboardEntity _inputKeyboardEntity;

 public:
  void add(uint16_t inputKey);

  void run();
};

class TerminalStateComponent {
 private:
  uint16_t _state;

 public:
  void setState(uint16_t state);

  uint16_t getState();
};

class WindowStateComponent {
 private:
  std::vector<void*> _stateBox;
  std::vector<void*>::iterator _stateIterator;

 public:
  template <class Box>
  void add(Box box);

  template <class Box>
  Box next();
  // std::vector<InputBoxEntity*> _stateInputBox;
  WindowStateComponent() { _stateIterator = _stateBox.begin(); }
};

#endif

