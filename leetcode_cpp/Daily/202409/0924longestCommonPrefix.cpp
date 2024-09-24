#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct Node {
    std::array<Node*, 10> children = {nullptr};
    bool isEnd = false;
};

struct Trie {
    Node dummy;

    void add(const std::string& s) {
        Node* cur = &dummy;
        for (auto c : s) {
            auto& next = cur->children[c - '0'];
            if (!next) {
                next = new Node();
            }
            cur = next;
        }
        cur->isEnd = true;
    }

    [[nodiscard]]
    int hasPrefix(const std::string& s) const {
        const Node* cur = &dummy;
        int ret = 0;
        for (auto c : s) {
            auto next = cur->children[c - '0'];
            if (!next) {
                break;
            }
            ret++;
            cur = next;
        }
        return ret;
    }
};

class Solution {
  public:
    static int longestCommonPrefix(vector<int>& arr1, vector<int>& arr2) {
        Trie t;
        for (auto v : arr2) {
            t.add(std::to_string(v));
        }
        int ans = 0;
        for (auto v : arr1) {
            std::string s = std::to_string(v);
            int n = s.size();
            if (n > ans) {
                ans = std::max(ans, t.hasPrefix(s));
            }
        }
        return ans;
    }
};