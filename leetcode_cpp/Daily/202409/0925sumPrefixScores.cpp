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
    static int calculatePrefix(const auto& s1, const auto& s2) {
        int n = std::min(s1.size(), s2.size());
        for (int i = 0; i < n; ++i) {
            if (s1[i] != s2[i]) {
                return i;
            }
        }
        return n;
    }

    static std::vector<int> sumPrefixScores(std::vector<std::string>& words) {
        int n = words.size();
        std::vector<size_t> indices(n);
        for (int i = 0; i < n; ++i) {
            indices[i] = i;
        }

        std::ranges::sort(indices, [&](int l, int r) { return words[l] < words[r]; });
        std::vector<int> prefixTable(n, 0);
        for (int i = 1; i < n; ++i) {
            auto& prevWord = words[indices[i - 1]];
            auto& currWord = words[indices[i]];
            prefixTable[i] = calculatePrefix(prevWord, currWord);
        }

        std::vector<int> scores(n, 0);
        for (int i = 0; i < n; ++i) {
            size_t idx = indices[i];
            int wordPrefix = words[idx].size();
            scores[idx] += wordPrefix;
            for (auto j = i + 1; j < n; ++j) {
                if (wordPrefix == 0) {
                    break;
                }
                size_t nextIdx = indices[j];
                wordPrefix = std::min(wordPrefix, prefixTable[j]);
                scores[nextIdx] += wordPrefix;
                scores[idx] += wordPrefix;
            }
        }
        return scores;
    }
};

struct Node {
    std::array<Node*, 26> children = {nullptr};
    std::vector<size_t> indices;
};

struct Trie {
    Node dummy;
    std::vector<int> scores;

    explicit Trie(int n):
      scores(n, 0) {}

    void insert(const std::string& word, size_t idx) {
        auto& score = scores[idx];
        score += word.size();
        auto* node = &dummy;
        for (auto c : word) {
            auto& child = node->children[c - 'a'];
            if (!child) {
                child = new Node();
            }
            child->indices.push_back(idx);
            node = child;
        }
    }

    void update(const std::string& word, size_t cur) {
        auto* node = &dummy;
        for (auto c : word) {
            node = node->children[c - 'a'];
            if (!node) {
                return;
            }
            for (auto idx : node->indices) {
                scores[idx]++;
                scores[cur]++;
            }
        }
    }
};

class MyTrieSlowSolution {
  public:
    static vector<int> sumPrefixScores(vector<string>& words) {
        Trie trie(words.size());
        for (size_t idx = 0; idx < words.size(); ++idx) {
            auto& word = words[idx];
            trie.update(word, idx);
            trie.insert(word, idx);
        }
        return std::move(trie.scores);
    }
};