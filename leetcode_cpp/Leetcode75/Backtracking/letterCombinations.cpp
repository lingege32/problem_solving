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
    static vector<string> letterCombinations(const string& digits) {
        if (digits.empty()) {
            return {};
        }
        std::array<std::string, 8> MAPPING = {"abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"};
        std::vector<std::string> ret = {""};
        std::vector<std::string> tmp;

        for (auto& digit : digits) {
            int n = digit - '2';
            auto& str = MAPPING[n];
            tmp.reserve(ret.size() * str.size());
            for (auto& back : ret) {
                for (auto c : str) {
                    tmp.push_back(back + c);
                }
            }
            tmp.swap(ret);
            tmp.clear();
        }

        return ret;
    }
};