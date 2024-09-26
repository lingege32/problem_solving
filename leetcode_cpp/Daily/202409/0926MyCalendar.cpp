#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class MyCalendar {
  public:
    std::map<int, int> mm;
    MyCalendar() = default;

    bool book(int start, int end) {
        auto it = mm.lower_bound(start);
        if (it != mm.end() && it->first < end) {
            return false;
        }

        if (it != mm.begin()) {
            --it;
            if (it->second > start) {
                return false;
            }
        }

        mm[start] = end;
        return true;
    }
};
