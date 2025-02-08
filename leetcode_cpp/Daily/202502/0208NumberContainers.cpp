#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

using MinHeap = priority_queue<int, vector<int>, greater<>>;

class NumberContainers {
    std::unordered_map<int, int> idxToNum;
    std::unordered_map<int, MinHeap> numCount;

  public:
    void change(int index, int number) {
        auto [it, inserted] = idxToNum.emplace(index, number);
        if (inserted) {
            numCount[number].push(index);
        } else if (it->second != number) {
            it->second = number;
            numCount[number].push(index);
        }
    }

    int find(int number) {
        auto it = numCount.find(number);
        if (it != numCount.end()) {
            auto& mh = it->second;
            while (!mh.empty()) {
                auto top = mh.top();
                if (idxToNum[top] == number) {
                    return top;
                }
                mh.pop();
            }
        }
        return -1;
    }
};

/**
 * Your NumberContainers object will be instantiated and called as such:
 * NumberContainers* obj = new NumberContainers();
 * obj->change(index,number);
 * int param_2 = obj->find(number);
 */