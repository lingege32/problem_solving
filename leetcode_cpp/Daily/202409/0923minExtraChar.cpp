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
    static int minExtraChar(string s, vector<string>& dictionary) {
        int const n = s.size();
        std::vector<int> dp(n + 1);

        for (int i = 1; i < n + 1; ++i) {
            dp[i] = dp[i - 1] + 1;
            for (std::string const& word : dictionary) {
                int const m = word.length();
                if (i >= m) {
                    bool flag = true;
                    for (int j = i - m; j < i; ++j) {
                        if (s[j] != word[j - i + m]) {
                            flag = false;
                            break;
                        }
                    }
                    if (flag) {
                        dp[i] = std::min(dp[i], dp[i - m]);
                    }
                }
            }
        }
        return dp[n];
    }
};

struct Node {
    std::array<Node*, 26> children = {nullptr};
    bool isEnd = false;
};

struct Trie {
    Node dummy;

    void add(const std::string& s) {
        auto cur = &dummy;
        for (auto c : s) {
            auto& next = cur->children[c - 'a'];
            if (!next) {
                next = new Node();
            }

            cur = next;
        }
        cur->isEnd = true;
    }

    [[nodiscard]] bool contains(const std::string& s) const {
        auto cur = &dummy;
        for (auto c : s) {
            auto next = cur->children[c - 'a'];
            if (!next) {
                return false;
            }
            cur = next;
        }
        return cur->isEnd;
    }
};

class TrieSolution {
  public:
    static int minExtraChar(const string& s, vector<string>& dictionary) {
        Trie tree;
        for (const auto& word : dictionary) {
            tree.add(word);
        }

        int n = s.size();
        std::vector<vector<int>> cache(n, vector<int>(n + 1, std::numeric_limits<int>::max()));
        return minExtraChar(s, 0, n, tree, cache);
    }

    static int minExtraChar(const std::string& s, int left, int right, const Trie& trie, vector<vector<int>>& cache) {
        auto& min = cache[left][right];
        if (min != std::numeric_limits<int>::max()) {
            return min;
        }
        auto sub = s.substr(left, right - left);
        if (trie.contains(sub)) {
            min = 0;
            return 0;
        }
        if (right - left == 1) {
            min = 1;
            return 1;
        }
        for (int mid = left + 1; mid < right; ++mid) {
            auto l = minExtraChar(s, left, mid, trie, cache);
            auto r = minExtraChar(s, mid, right, trie, cache);
            min = std::min(min, l + r);
        }
        return min;
    }
};