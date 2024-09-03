#include <array>
#include <string>
#include <vector>
using namespace std;
class Solution {
  public:
    string clearStars(string s) {
        std::array<std::vector<size_t>, 26> idx_collection;
        std::string ans;
        ans.resize(s.size(), 'n');
        for (size_t idx = 0; idx < s.size(); ++idx) {
            auto c = s[idx];
            if (c == '*') {
                ans[idx] = 'y';
                for(auto& v:idx_collection) {
                    if (!v.empty()) {
                        auto i = v.back();
                        v.pop_back();
                        ans[i] = 'y';
                        break;
                    }
                }
            } else {
                idx_collection[c - 'a'].push_back(idx);
            }
        }
        size_t i=0;
        for (size_t idx=0; idx<s.size(); ++idx) {
            if (ans[idx] == 'n') {
                ans[i] = s[idx];
                ++i;
            }
        }
        ans.resize(i);
        return ans;
    }
};