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
    static int prefixCount(vector<string>& words, const string& pref) {
        std::string_view pattern = pref;
        size_t len = pref.size();
        auto check = [&](std::string_view word) { return word.size() >= len && word.substr(0, len) == pattern; };
        return std::count_if(words.begin(), words.end(), check);
    }
};