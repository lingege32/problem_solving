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
    static int isPrefixOfWord(const string& sentence, const string& searchWord) {
        auto pos = sentence.find(searchWord);
        if (pos == string::npos) {
            return -1;
        }
        if (pos == 0) {
            return 1;
        }
        while (pos != std::string::npos && sentence[pos - 1] != ' ') {
            pos += searchWord.size();
            pos = sentence.find(searchWord, pos + 1);
        }

        return pos == std::string::npos ? -1 : std::count(sentence.begin(), sentence.begin() + pos, ' ') + 1;
    }
};