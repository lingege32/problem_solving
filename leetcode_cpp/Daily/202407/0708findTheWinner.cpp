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
    static int findTheWinner(int n, int k) {
        int winner = 0;
        for (int i = 1; i <= n; i++) {
            winner = (winner + k) % i;
        }
        return winner + 1;
    }
};

/*
n=4, k=2

1. 0 1 2 3 // index (2+2) % s is winner
2. 0 2 3 -> 2 3 0  // index (0+2) % 3 is winner
3. 2 0 -> 0 2 // index (0 + k)%n is winner
4. 0 // index 0 is winner

* after step 1, remove the index 1 from the vector and start from index 2
* if we rotate the index 2 to the first, we can say that it start from index 0;
* To think reversely, if we want to know who is the winner index of the array with n elements, we can take the winner index x with n-1 elements and shift right k steps.

*/