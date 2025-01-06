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
    static string shiftingLetters(string s, const vector<vector<int>>& shifts) {
        int n = s.size();
        std::vector<int> shifters(n + 1);
        for (auto& shift : shifts) {
            int dir = shift[2] == 0 ? -1 : 1;
            shifters[shift[0]] += dir;
            shifters[shift[1] + 1] -= dir;
        }
        int move = 0;
        for (auto i = 0; i < n; ++i) {
            move += shifters[i];
            if (move > 0) {
                s[i] = (s[i] - 'a' + move) % 26 + 'a';

            } else if (move < 0) {
                s[i] = 'z' - (('z' - s[i] - move)) % 26;
            }
        }
        return s;
    }
};