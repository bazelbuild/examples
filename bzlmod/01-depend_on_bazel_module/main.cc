#include <iostream>
#include "absl/strings/str_cat.h"

int main(int argc, char* argv[]) {
    int num_cookies = 42;
    std::string s = absl::StrCat("Found ", num_cookies, " cookies");
    std::cout << s << std::endl;
}
