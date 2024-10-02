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
    static string reverseWords(const string& s) {
        int n = s.size();
        std::vector<std::string> r;
        auto pos = s.find_first_not_of(' ');
        for (; pos != std::string::npos;) {
            auto end = s.find(' ', pos);
            int len = std::min(end - pos, n - pos);
            r.emplace_back(s.substr(pos, len));
            pos = s.find_first_not_of(' ', end);
        }
        std::string ret;
        if (r.empty()) {
            return ret;
        }
        auto it = r.rbegin();
        ret = std::move(*it);
        it++;
        for (; it != r.rend(); it++) {
            ret += " ";
            ret += *it;
        }
        return ret;
    }
};