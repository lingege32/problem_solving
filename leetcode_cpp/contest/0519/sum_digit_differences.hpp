#include <algorithm>
#include <iterator>
#include <vector>
#include <iostream>
#include <array>
#include <unordered_map>
using namespace std;
class Solution {
public:
    long long sumDigitDifferences(vector<int>& nums) {
        std::array<std::array<int,10>, 10> bits;
        for(auto n:nums) {
            convert(n, bits);
        }
        long long ans=0;
        for(const auto& bit: bits) {
            for(auto i = 1; i<10; ++i) {
                for(auto k=0;k<i;++k) {
                    ans+=bit[i]*bit[k];
                }
            }
        }
        return ans;
    }

    void convert(int a, std::array<std::array<int,10>, 10>& bits) {
        size_t idx=0;
        while(a>0) {
            auto r = a%10;
            bits[idx][r]++;
            a/=10;
            idx++;
        }
    }
};