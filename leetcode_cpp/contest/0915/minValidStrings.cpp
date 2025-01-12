#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static std::vector<int> prefix_of_target(std::string word, const std::string& target) {
        word += "#";
        word += target;
        int n = word.size();
        std::vector<int> pi(n, 0);
        for (int i = 1; i < n; ++i) {
            int j = pi[i - 1];
            while (j > 0 && word[i] != word[j]) {
                j = pi[j - 1];
            }
            if (word[i] == word[j]) {
                j++;
            }
            pi[i] = j;
        }

        return pi;
    }

    static int minValidStrings(vector<string>& words, const string& target) {
        // first step is to calculate the back N characters of index i of target.
        // can be a prefix.
        std::vector<int> back(target.size(), 0);
        for (auto& word : words) {
            auto pi = prefix_of_target(word, target);
            for (size_t i = 0; i < target.size(); ++i) {
                back[i] = std::max(back[i], pi[i + word.size() + 1]);
            }
        }

        if (std::ranges::any_of(back, [](int x) { return x == 0; })) {
            return -1;
        }

        // second step use dp to calculate
        std::vector<int> dp(target.size() + 1, 1e9);
        dp[0] = 0;
        for (size_t i = 0; i < target.size(); ++i) {
            dp[i + 1] = dp[i + 1 - back[i]] + 1;
        }
        return dp.back();
    }
};


// optimize for Hard.....
namespace {

constexpr int N = 100000;

struct node {
    int len, kmp;
    std::array<int, 26> child;
};

std::array<node, N> ac;
int tot;
std::array<int, N> que;

void insert(const char* s, int n) {
    int rt = 0;
    for (int len = 1; len <= n; ++len, ++s) {
        if (!ac[rt].child[*s - 'a']) {
            ac[ac[rt].child[*s - 'a'] = tot++] = {.len = len, .kmp = 0, .child = {}};
        }
        rt = ac[rt].child[*s - 'a'];
    }
}

void kmp() {
    int h = 0, n = 0;
    for (int i : ac[0].child) {
        if (i) {
            que[n++] = i;
        }
    }
    while (h < n) {
        int u = que[h++], kmp = ac[u].kmp;
        for (int i = 0; i < 26; ++i) {
            if (!ac[u].child[i]) {
                ac[u].child[i] = ac[kmp].child[i];
            } else {
                ac[ac[u].child[i]].kmp = ac[kmp].child[i];
                que[n++] = ac[u].child[i];
            }
        }
    }
}

struct Prefix {
    int last = 0;
    int flag = 0;
};

inline std::array<Prefix, 50000> prefix;
int FLAG;
}  // namespace

class OptSolution {
  public:
    static int minValidStrings(vector<string>& words, string& target) {
        tot = 1;
        ac[0] = {};
        for (auto& s : words) {
            insert(s.c_str(), min(s.length(), target.length()));
        }
        kmp();
        const int n = target.length();
        const char* s = target.c_str();
        ++FLAG;
        for (int i = 0, u = 0; i < n; ++i, ++s) {
            u = ac[u].child[*s - 'a'];
            if (u == 0) {
                return -1;
            }
            prefix[i + 1 - ac[u].len] = {.last = i, .flag = FLAG};
        }
        int ans = 0;
        for (int i = 0, last = -1, right = -1; i < n; ++i) {
            if (prefix[i].flag == FLAG) {
                right = max(right, prefix[i].last);
            }
            if (i > last) {
                ++ans;
                last = right;
            }
        }
        return ans;
    }
};
