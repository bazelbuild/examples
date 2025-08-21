#include <iostream>

std::string get_greet(const std::string& who) {
  return "Hello " + who;
}


int main(int argc, char** argv) {
  std::string who = "world";
  std::cout << get_greet(who) << std::endl;
  return 0;
}
