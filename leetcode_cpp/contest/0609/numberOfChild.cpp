class Solution {
public:
    int numberOfChild(int n, int k) {
        int m = 2*n-2;
        k %= m;
        if (k<n) {
            return k;
        } else {
            return m-k;
        }
    }
};