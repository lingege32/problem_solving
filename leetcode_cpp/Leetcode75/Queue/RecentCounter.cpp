#include <bits/stdc++.h>

using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class RecentCounter {
    std::queue<int> q;

  public:
    int ping(int t) {
        q.push(t);
        t -= 3000;
        while (q.front() < t) {
            q.pop();
        }
        return q.size();
    }
};

/**
 * Your RecentCounter object will be instantiated and called as such:
 * RecentCounter* obj = new RecentCounter();
 * int param_1 = obj->ping(t);
 */