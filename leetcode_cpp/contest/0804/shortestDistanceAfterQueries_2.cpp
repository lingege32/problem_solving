#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    vector<int> shortestDistanceAfterQueries(int n, vector<vector<int>>& queries) {
        std::map<int, int> mm;
        mm[queries[0][0]] = queries[0][1];
        int max = n - (queries[0][1] - queries[0][0]);

        vector<int> ans;
        ans.reserve(queries.size());
        ans.push_back(max);
        for (size_t idx = 1; idx < queries.size(); ++idx) {
            const auto& q = queries[idx];
            auto ql = q[0];
            auto qr = q[1];
            bool insert = true;

            auto it = mm.upper_bound(ql);
            if (it != mm.begin()) {
                it--;
                if (it->first <= ql && it->second >= qr) {
                    insert = false;
                }
            }
            if (insert) {
                it = mm.lower_bound(ql);
                // std::cout<<"---"<<ql<<", "<<qr<<std::endl;
                // for(auto p : mm) {
                //     std::cout<<p.first<<", "<<p.second<<std::endl;
                // }
                while (it != mm.end()) {
                    if (it->second <= qr) {
                        auto d = it->second - it->first - 1;
                        // std::cout<<"***"<<it->second <<", "<<it->first<<std::endl;
                        max += d;
                        it = mm.erase(it);
                    } else {
                        break;
                    }
                }
                mm[ql] = qr;
                max = max - (qr - ql) + 1;
            }
            ans.push_back(max);
        }

        return ans;
    }
};