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
    static string repeatLimitedString(const string& s, int repeatLimit) {
        std::array<int, 26> cnt;
        cnt.fill(0);
        for (auto c : s) {
            cnt[c - 'a']++;
        }
        std::string ret;
        ret.reserve(s.size());
        for (int i = 25; i >= 0; --i) {
            if (cnt[i] > 0) {
                ret.push_back('a' + i);
                cnt[i]--;
                break;
            }
        }

        auto take = [&](int n) {
            for (int i = 25; i >= 0; --i) {
                if (cnt[i] > 0) {
                    n--;
                    if (n == 0) {
                        return static_cast<char>('a' + i);
                    }
                }
            }
            return '\0';
        };
        int cur = 1;
        for (;;) {
            char c = take(1);
            if (cur == repeatLimit && c == ret.back()) {
                c = take(2);
            }
            if (c == '\0') {
                break;
            }
            if (c == ret.back()) {
                cur++;
            } else {
                cur = 1;
            }
            // std::cout<<c<<", "<<c-'a'<<std::endl;;
            cnt[c - 'a']--;
            ret.push_back(c);
        }
        return ret;
    }
};

class OptSolution {
  public:
    static string repeatLimitedString(const string& s, int repeatLimit) {
        vector<int> freq(26, 0);
        for (char ch : s) {
            freq[ch - 'a']++;
        }

        string result;
        int currentCharIndex = 25;  // Start from the largest character
        while (currentCharIndex >= 0) {
            if (freq[currentCharIndex] == 0) {
                currentCharIndex--;
                continue;
            }

            int use = min(freq[currentCharIndex], repeatLimit);
            result.append(use, 'a' + currentCharIndex);
            freq[currentCharIndex] -= use;

            if (freq[currentCharIndex] > 0) {  // Need to add a smaller character
                int smallerCharIndex = currentCharIndex - 1;
                while (smallerCharIndex >= 0 && freq[smallerCharIndex] == 0) {
                    smallerCharIndex--;
                }
                if (smallerCharIndex < 0) {
                    break;
                }
                result.push_back('a' + smallerCharIndex);
                freq[smallerCharIndex]--;
            }
        }

        return result;
    }
};