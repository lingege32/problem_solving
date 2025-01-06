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
    static vector<int> minOperations(const string& boxes) {
        int n = boxes.size();
        std::vector<int> ret(n, 0);
        int move = 0;
        int num = boxes[0] == '1';
        for (int i = 1; i < n; ++i) {
            ret[i] += (move + num);
            num += boxes[i] == '1';
            move = ret[i];
        }
        move = 0;
        num = boxes.back() == '1';
        for (int i = n - 2; i >= 0; --i) {
            auto ori = ret[i];
            ret[i] += (move + num);
            num += boxes[i] == '1';
            move = ret[i] - ori;
        }
        return ret;
    }
};