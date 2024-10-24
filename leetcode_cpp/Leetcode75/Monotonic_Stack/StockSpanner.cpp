#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class StockSpanner {
    std::vector<std::pair<int, int>> stack;

  public:
    int next(int price) {
        int ret = 1;
        while (!stack.empty() && stack.back().first <= price) {
            auto [p, span] = stack.back();
            stack.pop_back();
            ret += span;
        }
        stack.emplace_back(price, ret);
        return ret;
    }
};