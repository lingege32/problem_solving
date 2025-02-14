#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class ProductOfNumbers {
    std::vector<int> nums{1};

  public:
    void add(int num) {
        if (num != 0) {
            nums.push_back(num * nums.back());
        } else {
            nums.clear();
            nums.push_back(1);
        }
    }

    int getProduct(int k) { return k < static_cast<int>(nums.size()) ? nums.back() / nums[nums.size() - k - 1] : 0; }
};

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * ProductOfNumbers* obj = new ProductOfNumbers();
 * obj->add(num);
 * int param_2 = obj->getProduct(k);
 */