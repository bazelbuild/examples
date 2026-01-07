#include "absl/log/log.h"

int main(int argc, char* argv[]) {
    int num_cookies = 42;
    LOG(INFO) << "Found " << num_cookies << " cookies";
}
