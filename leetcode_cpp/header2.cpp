#include "header1.h"
#include <algorithm>

int Solution::maxDistToClosest(vector<int> &seats) {
    int left_sitting = 0;
    for (; seats[left_sitting] == 0; ++left_sitting) {
    }
    int max_distance = left_sitting;
    int ss = seats.size();
    for (int right_sitting = left_sitting + 1; right_sitting < ss;
         ++right_sitting) {
        if (seats[right_sitting] == 1) {
            max_distance =
                std::max(max_distance, (right_sitting - left_sitting) / 2);
            left_sitting = right_sitting;
        }
    }
    for (int i = ss - 1; i >= 0; --i) {
        if (seats[i] == 1) {
            max_distance = std::max(max_distance, ss - 1 - i);
            break;
        }
    }

    return max_distance;
}

Node *Solution::cloneGraph(Node *node) {
    unordered_set<Node *> done;
    unordered_map<Node *, Node *> table;
    queue<Node *> q;
    table[node] = new Node(node->val);
    q.push(node);

    while (!q.empty()) {
        Node *n = q.front();
        q.pop();
        if (done.find(n) != done.end()) {
            continue;
        } else {
            done.insert(n);
        }

        Node *new_cur = table[n];
        for (auto neighbor : n->neighbors) {
            q.push(neighbor);
            auto neiIter = table.find(neighbor);
            Node *new_neighbor = nullptr;
            if (neiIter == table.end()) {
                new_neighbor = new Node(neighbor->val);
                table[neighbor] = new_neighbor;
            } else {
                new_neighbor = neiIter->second;
            }
            new_cur->neighbors.push_back(new_neighbor);
        }
    }
    return table[node];
}

bool Solution::wordPattern(string pattern, string s) {
    unordered_map<char, string> m;
    istringstream in(s);
    int i = 0, n = pattern.size();
    for (string word; in >> word; ++i) {
        if (i >= n)
            continue;
        if (m.count(pattern[i])) {
            if (m[pattern[i]] != word)
                return false;
        } else {
            for (auto &a : m) {
                if (a.second == word)
                    return false;
            }
            m[pattern[i]] = word;
        }
    }
    return i == n;
}

ListNode *Solution::detectCycle(ListNode *head) {
    ListNode *slow = head;
    ListNode *fast = head;
    while (fast) {
        slow = slow->next;
        fast = fast->next;
        if (fast) {
            fast = fast->next;
        }
        if (fast && slow == fast) {
            break;
        }
    }
    if (fast) {
        slow = head;
        while (slow != fast) {
            slow = slow->next;
            fast = fast->next;
        }
        return slow;
    }
    return nullptr;
}
bool Solution::canPlaceFlowers(vector<int> &flowerbed, int n) {
    auto it1 = std::find(flowerbed.begin(), flowerbed.end(), 1);
    if (it1 == flowerbed.end())
        return (flowerbed.size() + 1) / 2 >= n;
    int max_flowers = std::distance(flowerbed.begin(), it1) / 2;

    auto it0 = std::find(it1, flowerbed.end(), 0);
    it1 = std::find(it0, flowerbed.end(), 1);
    while (it1 != flowerbed.end()) {
        max_flowers += (std::distance(it0, it1) - 1) / 2;
        it0 = std::find(it1, flowerbed.end(), 0);
        it1 = std::find(it0, flowerbed.end(), 1);
    }
    max_flowers += (std::distance(it0, it1) / 2);
    return max_flowers >= n;
}

int Solution::minEatingSpeed(vector<int> &piles, int h) {
    int left = 1;
    int right = *std::max_element(piles.begin(), piles.end());
    while (left < right) {
        int mid = (left + right) / 2;
        int needHr = 0;
        for (auto banana : piles) {

            needHr += ((banana / mid) + ((banana % mid) == 0 ? 0 : 1));
            if (needHr > h) {
                break;
            }
        }
        if (needHr > h) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    return left;
}

int Solution::canCompleteCircuit(vector<int> &gas, vector<int> &cost) {
    const int n = gas.size();
    int sum = 0, min_sum = std::numeric_limits<int>::max(), min_i = -1;
    for (int i = 0; i < n; ++i) {
        sum += gas[i] - cost[i];
        if (sum < min_sum) {
            min_sum = sum;
            min_i = i;
        }
    }
    return sum >= 0 ? ((min_i + 1) % n) : -1;
}