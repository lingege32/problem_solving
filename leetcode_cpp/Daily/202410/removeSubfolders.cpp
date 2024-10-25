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
    static vector<string> removeSubfolders(vector<string>& folder) {
        std::ranges::sort(folder);

        std::vector<std::string> ans;
        ans.reserve(folder.size());
        ans.push_back(std::move(folder.front()));

        size_t i{1};
        while (i < folder.size()) {
            if (!folder[i].starts_with(ans.back() + '/')) {
                ans.push_back(std::move(folder[i]));
            }
            ++i;
        }

        return ans;
    }
};