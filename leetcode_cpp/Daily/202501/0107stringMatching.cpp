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
    static vector<string> stringMatching(vector<string>& words) {
        int n = words.size();
        std::vector<string> ret;
        for (int i = 0; i < n; ++i) {
            auto& sub = words[i];
            for (int j = 0; j < n; ++j) {
                if (i == j) {
                    continue;
                }
                auto& other = words[j];
                if (sub.size() >= other.size()) {
                    continue;
                }
                if (words[j].find(sub) != string::npos) {
                    ret.push_back(sub);
                    break;
                }
            }
        }
        return ret;
    }
};