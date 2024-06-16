#include <vector>
using namespace std;
class Solution {
public:
    int countCompleteDayPairs(vector<int>& hours) {
        int count=0;
        for(size_t idx=0; idx<hours.size(); ++idx) {
            for (size_t i=idx+1; i<hours.size(); ++i) {
                if ((hours[idx] + hours[i]) % 24 == 0 ) {
                    count++;
                }
            }
        }
        return count;
    }
};