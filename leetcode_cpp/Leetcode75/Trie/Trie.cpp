#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Trie {
    std::array<Trie*, 26> children;
    bool isEnd = false;

  public:
    Trie() { children.fill(nullptr); }

    void insert(const string& word) {
        auto* node = this;
        for (auto c : word) {
            if (node->children[c - 'a'] == nullptr) {
                node->children[c - 'a'] = new Trie();
            }
            node = node->children[c - 'a'];
        }
        node->isEnd = true;
    }

    bool search(const string& word) {
        auto* node = this;
        for (char c : word) {
            if (node->children[c - 'a'] == nullptr) {
                return false;
            }
            node = node->children[c - 'a'];
        }
        return node->isEnd;
    }

    bool startsWith(const string& word) {
        auto* node = this;
        for (char c : word) {
            if (node->children[c - 'a'] == nullptr) {
                return false;
            }
            node = node->children[c - 'a'];
        }
        return true;
    }
};
