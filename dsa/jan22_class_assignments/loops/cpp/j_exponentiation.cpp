#include <iostream>
using namespace std;

int main() {
    int base, exponent;
    cin >> base >> exponent;
    long long exponential = 1;
    while (exponent--) {
        exponential *= base;
    }
    cout << exponential << endl;
}
