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
    std::vector<std::string> words;

  public:
    Trie() { children.fill(nullptr); }

    void insert(const string& word) {
        auto* node = this;
        for (auto c : word) {
            if (node->children[c - 'a'] == nullptr) {
                node->children[c - 'a'] = new Trie();
            }
            node = node->children[c - 'a'];
            if (node->words.size() < 3) {
                node->words.push_back(word);
            }
        }
    }

    std::vector<std::string> search(const string& prefix) {
        auto* node = this;
        for (char c : prefix) {
            if (node->children[c - 'a'] == nullptr) {
                return {};
            }
            node = node->children[c - 'a'];
        }
        return node->words;
    }
};

class Solution {
  public:
    static vector<vector<string>> suggestedProducts(vector<string>& products, const string& searchWord) {
        std::ranges::sort(products);
        Trie trie;
        for (const auto& product : products) {
            trie.insert(product);
        }
        std::vector<std::vector<std::string>> ret;
        ret.reserve(searchWord.size());
        for (size_t i = 0; i < searchWord.size(); ++i) {
            std::string prefix = searchWord.substr(0, i + 1);
            ret.push_back(trie.search(prefix));
        }

        return ret;
    }
};

class OptSolution {
  public:
    static vector<vector<string>> suggestedProducts(vector<string>& products, const string& searchWord) {
        vector<vector<string>> result;
        std::ranges::sort(products);
        string word;
        int indexShift = 0;
        for (char const& ch : searchWord) {
            vector<string> wordVec;
            word += ch;
            auto start = lower_bound(products.begin() + indexShift, products.end(), word);
            indexShift = start - products.begin();
            for (auto i = start; i < min(start + 3, products.end()); i++) {
                if ((*i).starts_with(word)) {
                    wordVec.push_back(*i);
                }
            }
            result.push_back(wordVec);
        }
        return result;
    }
};