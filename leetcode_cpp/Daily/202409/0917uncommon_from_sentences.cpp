#include <bits/stdc++.h>
using namespace std;

class Solution final {
  public:
    static std::vector<std::string> uncommonFromSentences(const std::string& s1, const std::string& s2)  noexcept {
        std::unordered_map<std::string, size_t> counter;
        std::stringstream ss1(s1);
        std::string word;
        while (ss1 >> word) {
            ++counter[word];
        }
        std::stringstream ss2(s2);
        while (ss2 >> word) {
            ++counter[word];
        }
        std::vector<std::string> uncommons;
        uncommons.reserve(counter.size() + 10);
        for (const auto& [w, count] : counter) {
            if (count == 1) {
                uncommons.push_back(w);
            }
        }
        return uncommons;
    }
};