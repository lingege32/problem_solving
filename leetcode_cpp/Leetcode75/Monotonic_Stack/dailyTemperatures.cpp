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
    static vector<int> dailyTemperatures(vector<int>& temperatures) {
        std::vector<int> stack;
        int n = temperatures.size();
        std::vector<int> ret(n, 0);
        for (int i = 0; i < n; ++i) {
            while (!stack.empty() && temperatures[i] > temperatures[stack.back()]) {
                auto idx = stack.back();
                stack.pop_back();
                ret[idx] = i - idx;
            }
            stack.push_back(i);
        }

        return ret;
    }
};