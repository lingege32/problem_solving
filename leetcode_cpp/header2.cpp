#include "header1.h"

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
            for (auto& a : m) {
                if (a.second == word)
                    return false;
            }
            m[pattern[i]] = word;
        }
    }
    return i == n;
}