#include <iostream>
using namespace std;

int main()
{
    long long num;
    cin >> num;
    long long sum = 0;
    do {
        sum += num % 10;
        num /= 10;
    } while (num > 0);

    cout << sum << endl;
}
