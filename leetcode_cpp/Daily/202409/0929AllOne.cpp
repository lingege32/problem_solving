#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class AllOne {
    std::unordered_map<std::string, int> mm;
    std::vector<unordered_set<std::string>> mm2;

  public:
    void inc(const string& key) {
        auto& val = mm[key];
        if (val) {
            mm2[val].erase(key);
        }
        val++;
        while (val >= static_cast<int>(mm2.size())) {
            mm2.emplace_back();
        }
        mm2[val].insert(key);

    }

    void dec(const string& key) {
        auto& val = mm[key];
        mm2[val].erase(key);
        val--;
        if (val) {
            mm2[val].insert(key);
        }

    }

    string getMaxKey() {
        for (auto& s : std::ranges::reverse_view(mm2)) {
            if (!s.empty()) {
                return *s.begin();
            }
        }
        return "";
    }

    string getMinKey() {
        for (const auto& s : mm2) {
            if (!s.empty()) {
                return *s.begin();
            }
        }
        return "";
    }
};
