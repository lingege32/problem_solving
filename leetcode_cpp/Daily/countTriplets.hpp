#include <algorithm>
#include <vector>
using namespace std;
class Solution {
  public:
    int countTriplets(vector<int> &arr) {
        vector<int> cache;
        auto cur = 0;
        for (auto it = arr.rbegin(); it != arr.rend(); ++it) {
            cur ^= *it;
            cache.push_back(cur);
        }
        std::reverse(cache.begin(), cache.end());
        int count = 0;
        for (unsigned k = 1; k < arr.size(); ++k) {
            auto ck = cache[k];
            for (unsigned i = 0; i < k; ++i) {
                auto ci = cache[i];
                for (unsigned j = i + 1; j < k; ++j) {
                    auto cj = cache[j];
                    auto a = ci ^ cj;
                    auto b = ck ^ cj;
                    if (a == b) {
                        // std::cout<<i<<", "<<j<<", "<<k<<"\n";
                        count++;
                    }
                }
                if ((ci ^ ck) == ck) {
                    count++;
                }
            }
        }
        return count;
    }
};

class Solution2 {
public:
    int countTriplets(vector<int>& A) {
        A.insert(A.begin(), 0);
        int n = A.size(), res = 0;
        for (int i = 1; i < n; ++i)
            A[i] ^= A[i - 1];
        for (int i = 0; i < n; ++i)
            for (int j = i + 1; j < n; ++j)
                if (A[i] == A[j])
                    res += j - i - 1;
        return res;
    }
};