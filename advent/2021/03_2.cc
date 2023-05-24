#include <iostream>
#include <string>
#include <set>
#include <functional>

int shit(std::set<std::string> &pool, std::function<bool(int, int)> cmp) {
    for (int i = 0; i < 12 && pool.size() > 1; i++) {
        int count_one = std::count_if(pool.begin(), pool.end(), [&](std::string s) {
            return s[i] == '1';
        });
        char least_common = cmp(count_one, pool.size()-count_one) ? '0' : '1';
        for (auto it = pool.begin(); it != pool.end(); ) {
            if ((*it)[i] == least_common) {
                it = pool.erase(it);
            } else {
                ++it;
            }
        }
    }
    if (pool.size() == 1) {
        return std::stoi(*(pool.begin()), nullptr, 2);
    }
    return 0;
}

int main(void) {
    std::set<std::string> pool;
    for (std::string line; std::getline(std::cin, line);) {
        pool.insert(line);
    }
    std::set<std::string> pool_copy(pool.begin(), pool.end());
    int sanso = shit(pool, [](int a, int b) { return a >= b; });
    int nisankatanso = shit(pool_copy, [](int a, int b) { return a < b; });
    std::cout << sanso * nisankatanso << std::endl;
}