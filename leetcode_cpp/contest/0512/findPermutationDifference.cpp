#include <string>
#include <unordered_map>
using namespace std;
class Solution {
  public:
    int findPermutationDifference(string s, string t) {
        unordered_map<char, int> table;
        for (size_t idx = 0; idx < s.size(); ++idx) {
            table[s[idx]] = idx;
        }
        int ret = 0;
        for (int idx = 0; idx < static_cast<int>(t.size()); idx++) {
            ret += std::abs(idx - table[t[idx]]);
        }
        return ret;
    }
};