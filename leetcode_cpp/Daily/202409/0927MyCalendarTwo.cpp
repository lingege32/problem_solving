#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class OtherMyCalendarTwo {
    vector<pair<int, int>> b;
    vector<pair<int, int>> db;

  public:
    bool book(int start, int end) {
        for (pair<int, int> x : db) {
            if (start < x.second && end > x.first) {
                return false;
            }
        }
        for (pair<int, int> x : b) {
            if (start < x.second && end > x.first) {
                db.emplace_back(max(start, x.first), min(end, x.second));
            }
        }
        b.emplace_back(start, end);
        return true;
    }
};

class MyCalendarTwo {
  public:
    constexpr static int THRESHOLD = 2;
    std::map<int, std::pair<int, int>> mm;

    bool check(auto it, int start, int end) {
        for (; it != mm.end(); it++) {
            if (it->first >= end) {
                break;
            }
            if (start < it->second.first && it->second.second == THRESHOLD) {
                return false;
            }
        }
        return true;
    }

    bool book(int start, int end) {
        auto it = mm.lower_bound(start);
        if (it != mm.begin()) {
            --it;
        }
        // std::cout << "Book: " << start << ", " << end << " ";
        auto c = check(it, start, end);
        if (!c) {
            // std::cout << "Failed.\n";
            return false;
        }

        for (; it != mm.end() && start != end; ++it) {
            int seg_start = it->first;
            if (seg_start >= end) {
                break;
            }

            int seg_end = it->second.first;
            int curBook = it->second.second;
            if (seg_start > start) {
                it = mm.try_emplace(start, seg_start, 1).first;
                start = seg_start;
            } else if (seg_start == start) {
                if (seg_end <= end) {
                    it->second.second++;
                    start = seg_end;
                } else {
                    it->second.second++;
                    it->second.first = end;
                    mm.try_emplace(end, seg_end, curBook);
                    break;
                }
            } else {
                if (seg_end > start) {
                    it->second.first = start;
                    if (end >= seg_end) {
                        it = mm.try_emplace(start, seg_end, curBook + 1).first;

                    } else {
                        mm.try_emplace(start, end, curBook + 1);
                        it = mm.try_emplace(end, seg_end, curBook).first;
                    }
                    start = std::min(end, seg_end);
                }
            }
        }
        if (start != end) {
            mm.try_emplace(start, end, 1);
        }
        // std::cout << "Success.\n";
        // for (auto [start, mapped] : mm) {
        //     auto [end, count] = mapped;
        //     std::cout << "[" << start << ", " << end << "]: " << count << std::endl;
        // }
        return true;
    }
};
