#include <iostream>
#include <string>

int main(int argc, const char* argv[]) {
  // Send argv to stdout.
  std::string divider(80, '#');
  std::cout << '\n' << divider << '\n';
  for (int i = 0; i < argc; i++) {
    std::cout << "MYTEST ARGV[" << i << "]: " << argv[i] << '\n';
  }
  std::cout << divider << '\n' << '\n';

  return 0;  // This test always passes.
}
