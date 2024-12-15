#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct Info {
    double increase = 0.0F;
    std::pair<int, int> passTotalRatio;
    bool operator<(const Info& other) const {
        return increase < other.increase;
    }
};

class Solution {
  public:
    static double maxAverageRatio(vector<vector<int>>& classes, int extraStudents) {
        int n = classes.size();
        std::priority_queue<Info, std::vector<Info>, std::less<>> pq;
        double ans = 0;
        for (auto& cls : classes) {
            auto pass = cls[0];
            auto total = cls[1];
            ans += static_cast<double>(pass) / total;
            double increase = (static_cast<double>(pass + 1) / (total + 1)) - (static_cast<double>(pass) / total);
            auto a = Info{.increase=increase, .passTotalRatio={pass, total}};
            pq.push(std::move(a));
        }
        ans/=n;

        while (extraStudents--) {
            auto [increase, passTotal] = pq.top();
            pq.pop();
            auto [pass, total] = passTotal;
            pass++,total++;
            ans += (increase/n);
            double newIncrease = (static_cast<double>(pass + 1) / (total + 1)) - (static_cast<double>(pass) / total);
            auto a = Info{.increase=newIncrease, .passTotalRatio={pass, total}};
            pq.push(std::move(a));
        }
        return ans;
    }
};