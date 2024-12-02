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
    static bool checkIfExist(vector<int>& arr) {
        std::unordered_set<int> s;
        for (auto i : arr) {
            if (s.contains(i * 2) || (i % 2 == 0 && s.contains(i / 2))) {
                return true;
            }
            s.insert(i);
        }
        return false;
    }
};