#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static bool canChange(const string& start, const string& target) {
        size_t start_pos = 0, target_pos = 0;

        for (;;) {
            auto start_next = start.find_first_not_of('_', start_pos);
            auto target_next = target.find_first_not_of('_', target_pos);
            if (start_next == std::string::npos && target_next == std::string::npos) {
                return true;
            }
            if (start_next == std::string::npos || target_next == std::string::npos) {
                return false;
            }
            // std::cout<<"s: "<<start_next<<", t: "<<target_next<<"\n";
            auto s = start[start_next];
            auto t = target[target_next];
            if (s != t) {
                return false;
            }
            if (s == 'L' && start_next < target_next) {
                return false;
            }
            if (s == 'R' && start_next > target_next) {
                return false;
            }
            start_pos = start_next + 1;
            target_pos = target_next + 1;
        }
        std::unreachable();
    }
};