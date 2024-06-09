#include <string>
#include <vector>
#include <iostream>
using namespace std;
class Solution {
  public:
    string compressedString(string word) {
        size_t base = 0;
        size_t len = 0;
        std::vector<std::pair<int, char>> collect;
        while (base != string::npos) {
            char cur = word[base];
            auto next = word.find_first_not_of(cur, base);
            // std::cout<<"base: "<<base<<", next: "<<next<<"\n";
            auto l = std::min(next, word.size()) - base;
            collect.emplace_back(l, cur);
            len += (l + 8) / 9;
            base = next;
        }
        // for (const auto &[count, c] : collect) {
        //     std::cout<<"count: "<<count<<", c: "<<c<<"\n";
        // }
        std::string ans;
        ans.reserve(len);

        for (const auto &[count, c] : collect) {
            int n9 = count / 9;
            for (int n = 0; n < n9; ++n) {
                ans += '9';
                ans += c;
            }
            auto r = count % 9;
            if (r != 0) {
                ans += ('0' + r);
                ans += c;
            }
        }
        return ans;
    }
};