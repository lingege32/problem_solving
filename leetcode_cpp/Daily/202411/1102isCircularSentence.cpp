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
    static bool isCircularSentence(string sentence) {
        bool ans = sentence.back() == sentence.front();
        auto pos = sentence.find(' ');
        for (; pos != std::string::npos && ans;) {
            ans = sentence[pos - 1] == sentence[pos + 1];
            pos = sentence.find(' ', pos + 1);
        }

        return ans;
    }
};