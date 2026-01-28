#include <iostream>
using namespace std;

int main() {
    long long num;
    cin >> num;
    long long original = num;
    long long reverse = 0;
    do {
        reverse = (reverse * 10) + (num % 10);
        num /= 10;
    } while (num > 0);

    if (reverse == original) {
        cout << "YES" << endl;
    } else {
        cout << "NO" << endl;
    }
}
