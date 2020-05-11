#include <iostream>

void runlib() {
#ifdef feature1
  std::cout << "Building lib with feature 1!\n";
#elif feature2
  std::cout << "Building lib with feature 2!\n";
#else
  std::cout << "Building lib without features. But the select() in the BUILD "
            << "file should prevent this case from building in the first "
            << "place.\n";
#endif
}
