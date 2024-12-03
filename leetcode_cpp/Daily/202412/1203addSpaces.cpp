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
    static string addSpaces(const string& s, const vector<int>& spaces) {
        auto new_size = s.size() + spaces.size();
        std::string ret;
        ret.reserve(new_size);
        size_t pos = 0;
        for (auto space_idx : spaces) {
            ret.insert(ret.end(), s.begin() + pos, s.begin() + space_idx);
            ret.push_back(' ');
            pos = space_idx;
        }
        ret.insert(ret.end(), s.begin() + pos, s.end());
        return ret;
    }
};