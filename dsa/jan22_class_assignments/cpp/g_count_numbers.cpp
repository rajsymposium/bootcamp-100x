#include <iostream>
using namespace std;

int main() {
    int nums;
    cin >> nums;
    int positive = 0, negative = 0, even = 0, odd = 0;

    for (int i = 0; i < nums; ++i) {
        int num;
        cin >> num;

        if (num < 0) {
            ++negative;
        } else if (num > 0) {
            ++positive;
        }
        if (num % 2 == 0) {
            ++even;
        } else {
            ++odd;
        }
    }

    cout << positive << endl;
    cout << negative << endl;
    cout << even << endl;
    cout << odd << endl;
}
