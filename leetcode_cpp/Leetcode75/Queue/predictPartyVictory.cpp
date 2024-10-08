#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
public:
    static string predictPartyVictory(string senate) {
        std::queue<size_t> rQueue;
        std::queue<size_t> dQueue;
        for (size_t i = 0; i < senate.size(); ++i)
        {
            if (senate[i] == 'R')
            {
                rQueue.push(i);
            }
            else
            {
                dQueue.push(i);
            }
        }

        while (!rQueue.empty() && !dQueue.empty())
        {
            int rqueue_front = rQueue.front(), dqueue_front = dQueue.front();
            rQueue.pop();
            dQueue.pop();
            if (rqueue_front < dqueue_front)
            {
                rQueue.push(rqueue_front + senate.size());
            }
            else
            {
                dQueue.push(dqueue_front + senate.size());
            }
        }

        return rQueue.empty() ? "Dire" : "Radiant";
    }
};