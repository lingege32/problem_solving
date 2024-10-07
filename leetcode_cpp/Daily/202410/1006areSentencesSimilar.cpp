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
    static bool areSentencesSimilar(const string& sentence1, const string& sentence2) {
        auto s1 = split(sentence1);
        auto s2 = split(sentence2);
        if (s1.size() > s2.size()) {
            return inner(s1, s2);
        }
        return inner(s2, s1);
    }

    static bool inner(const std::vector<std::string>& words1, const std::vector<std::string>& words2) {
        int start = 0, end = 0;
        int n1 = words1.size(), n2 = words2.size();

        // Compare from the start
        while (start < n2 && words1[start] == words2[start]) {
            start++;
        }

        // Compare from the end
        while (end < n2 && words1[n1 - end - 1] == words2[n2 - end - 1]) {
            end++;
        }

        // Check if the remaining unmatched part is in the middle
        return start + end >= n2;
    }

    static std::vector<std::string> split(const std::string& str) {
        std::vector<std::string> ret;
        auto pos = str.find_first_not_of(' ');
        while (pos != std::string::npos) {
            auto end = str.find(' ', pos);
            ret.push_back(str.substr(pos, end - pos));
            pos = str.find_first_not_of(' ', end);
        }
        return ret;
    }
};