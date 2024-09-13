#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static vector<int> xorQueries(vector<int>& arr, vector<vector<int>>& queries) {
        for (size_t i = 1; i < arr.size(); ++i) {
            arr[i] ^= arr[i - 1];
        }
        std::vector<int> ret;
        ret.reserve(queries.size());
        for (const auto& q : queries) {
            ret.push_back([&]() {
                auto r = arr[q[1]];
                auto l_idx = q[0];
                if (l_idx == 0) {
                    return r;
                }
                return r ^ arr[l_idx - 1];
            }());
        }
        return ret;
    }
};