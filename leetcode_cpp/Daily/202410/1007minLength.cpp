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
    static int minLength(string s) {
        vector<char> st;
        int n = s.size();
        for (int i = 0; i < n; i++) {
            if (!st.empty() && ((st.back() == 'A' && s[i] == 'B') ||

                                (st.back() == 'C' && s[i] == 'D')

                                    )) {
                st.pop_back();
            } else {
                st.push_back(s[i]);
            }
        }
        return st.size();
    }
};