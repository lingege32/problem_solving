#include <bits/stdc++.h>
#include <numeric>
using namespace std;
class Solution {
public:
  int countGoodNodes(vector<vector<int>> &edges) {
    unordered_map<int, vector<int>> edges_mapping;
    for (const auto &edge : edges) {
      edges_mapping[edge[0]].push_back(edge[1]);
      edges_mapping[edge[1]].push_back(edge[0]);
    }
    int ret = 0;
    unordered_set<int> visit;
    helpper(ret, 0, edges_mapping, visit);
    return ret;
  }

  int helpper(int &ans, int node, const unordered_map<int, vector<int>> &edges,
              unordered_set<int> &visit) {
    visit.insert(node);
    std::vector<int> child_total;
    for (const auto &to : edges.at(node)) {
      if (visit.find(to) == visit.end()) {
        child_total.push_back(helpper(ans, to, edges, visit));
      }
    }
    if (child_total.size() < 2 ||
        std::all_of(std::next(child_total.begin(), 1), child_total.end(),
                    [&](int x) { return x == child_total[0]; })) {
      ans++;
    }

    return 1 + std::accumulate(child_total.begin(), child_total.end(), 0);
  }
};