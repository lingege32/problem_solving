#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class MyCircularDeque {
    std::vector<int> array;
    int head = 0;
    int tail = 0;

  public:
    explicit MyCircularDeque(int k) { array.resize(k, -1); }

    bool insertFront(int value) {
        if (isFull()) {
            return false;
        }
        head = (head - 1 + array.size()) % array.size();
        array[head] = value;
        return true;
    }

    bool insertLast(int value) {
        if (isFull()) {
            return false;
        }
        array[tail] = value;
        tail = (tail + 1) % array.size();
        return true;
    }

    bool deleteFront() {
        if (isEmpty()) {
            return false;
        }
        array[head] = -1;
        head = (head + 1) % array.size();
        return true;
    }

    bool deleteLast() {
        if (isEmpty()) {
            return false;
        }
        tail = (tail - 1 + array.size()) % array.size();
        array[tail] = -1;
        return true;
    }

    int getFront() { return array[head]; }

    int getRear() { return array[(tail - 1 + array.size()) % array.size()]; }

    bool isEmpty() { return head == tail && array[head] == -1; }

    bool isFull() { return head == tail && array[head] != -1; }
};
