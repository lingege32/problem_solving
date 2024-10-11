#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class SmallestInfiniteSet {
    std::priority_queue<int, vector<int>, std::greater<>> q;
    int min = 1;

  public:
    int popSmallest() {
        if (q.empty()) {
            return min++;
        }
        auto t = q.top();
        while (!q.empty() && t == q.top()) {
            q.pop();
        }
        return t;
    }

    void addBack(int num) {
        if (num < min) {
            q.push(num);
        }
    }
};