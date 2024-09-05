#include <bits/stdc++.h>
using namespace std;
class Solution {
public:
    vector<int> resultsArray(vector<vector<int>>& queries, int k) {
        auto queries_size = queries.size();
        if (static_cast<int>(queries_size) < k) {
            return std::vector<int>(queries_size, -1);
        }
        std::map<int,int> m;
        std::vector<int> results;
        results.reserve(queries.size());
        for(int idx=0; idx<k-1; ++idx) {
            results.push_back(-1);
            m[std::abs(queries[idx][0]) + std::abs(queries[idx][1])]++;
        }
        m[std::abs(queries[k-1][0]) + std::abs(queries[k-1][1])]++;
        results.push_back(m.rbegin()->first);
        for(int idx=k; idx<queries_size; ++idx) {
            auto dis = std::abs(queries[idx][0]) + std::abs(queries[idx][1]);
            auto back = m.rbegin()->first;
            if (back <= dis) {
                results.push_back(back);
            } else {
                auto rb = m.rbegin();
                if (rb->second == 1) {
                    m.erase(rb->first);
                } else {
                    rb->second--;
                }
                m[dis]++;
                results.push_back(m.rbegin()->first);
            }
        }

        return results;
    }
};