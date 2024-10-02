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
    static int maxOperations(vector<int>& nums, int k) {
        std::unordered_map<int, int> mapping;
        int op = 0;
        for (auto num : nums) {
            int retain = k - num;
            auto it = mapping.find(retain);
            if (it != mapping.end()) {
                op++;
                if (it->second == 1) {
                    mapping.erase(it);
                } else {
                    it->second--;
                }
            } else {
                mapping[num]++;
            }
        }
        return op;
    }
};