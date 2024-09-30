#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class CustomStack {
    std::vector<int> m;

  public:
    explicit CustomStack(int maxSize) { m.reserve(maxSize); }

    void push(int x) {
        if (m.size() < m.capacity()) {
            m.push_back(x);
        }
    }

    int pop() {
        if (m.empty()) {
            return -1;
        }
        auto ret = m.back();
        m.pop_back();
        return ret;
    }

    void increment(int k, int val) {
        for (int i = 0; i < std::min(k, (int)m.size()); i++) {
            m[i] += val;
        }
    }
};