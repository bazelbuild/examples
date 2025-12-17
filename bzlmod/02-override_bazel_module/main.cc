#include <iostream>
#include "lib_a.h"
#include "re2/re2.h"

int main(int argc, char* argv[]) {
    std::cout << "Hello from the main module!" << std::endl;
    lib_a();

    if (RE2::FullMatch("bazel", "b.*l")) {
        std::cout << "RE2 match!" << std::endl;
    }
}
