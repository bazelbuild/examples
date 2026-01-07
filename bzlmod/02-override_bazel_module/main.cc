#include "absl/log/log.h"

#include "lib_a.h"

int main(int argc, char* argv[]) {
    LOG(INFO) << "Hello from the main module!";
    lib_a();
}
