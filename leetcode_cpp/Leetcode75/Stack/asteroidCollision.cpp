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
    static vector<int> asteroidCollision(const vector<int>& asteroids) {
        std::vector<int> ret;
        for (auto i : asteroids) {
            if (i < 0) {
                int j = -i;
                while (!ret.empty() && ret.back() >= 0 && ret.back() < j) {
                    ret.pop_back();
                }
                if (ret.empty() || ret.back() < 0) {
                    ret.push_back(i);
                } else if(ret.back() == j) {
                    ret.pop_back();
                }
            } else {
                ret.push_back(i);
            }
        }
        return ret;
    }
};