project('msg-terminal', 'cpp')

# Install curses
curses = dependency('ncurses', required: false, disabler: true)
dep_menu = dependency('menuw', required : false)
if not curses.found()
  curses_root = get_option('curses_root')
  curses_lib = cc.find_library('curses', dirs : curses_root, required : false, disabler: true)
  curses = declare_dependency(include_directories: curses_root, dependencies: curses_lib)
endif
if dep_menu.found() != true
	dep_menu = dependency('menu', required : true)
endif

executable('msg-terminal', 'main.cpp', dependencies: curses)