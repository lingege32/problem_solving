#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct Pair {
    int first = -1;
    int second = -1;
};

class Solution {
  public:
    static int cal(int num) {
        int sum = 0;
        while (num) {
            sum += num % 10;
            num /= 10;
        }
        return sum;
    }

    static int maximumSum(vector<int>& nums) {
        std::vector<Pair> table(82);
        for (auto num : nums) {
            auto val = cal(num);
            auto& p = table[val];
            if (num > p.first) {
                p.second = p.first;
                p.first = num;
            } else if (num > p.second) {
                p.second = num;
            }
        }
        int ret = -1;
        for (auto& pair : table) {
            if (pair.first != -1 && pair.second != -1) {
                ret = std::max(ret, pair.first + pair.second);
            }
        }
        return ret;
    }
};