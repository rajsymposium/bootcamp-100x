#include <iostream>
using namespace std;

int main() {
    int num;
    cin >> num;
    long long sum = 0;
    for (int i = 1; i <= num; ++i) {
        sum += i;
    }
    cout << sum << endl;
}
