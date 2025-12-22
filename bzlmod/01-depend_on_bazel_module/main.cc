#include <ng-log/logging.h>

int main(int argc, char* argv[]) {
    // Initialize Google’s logging library.
    nglog::InitializeLogging(argv[0]);

    int num_cookies = 42;
    LOG(INFO) << "Found " << num_cookies << " cookies";
}
