# This find all *.cpp/*.h/... files and format to google style
find . -regex '.*\.\(cpp\|hpp\|cc\|cxx\)' -exec clang-format -style=google -i {} \;
