#include <iostream>
using namespace std;

int main() {
    int rows, cols;
    cin >> rows >> cols;

    while (rows--) {
        for (int i = 0; i < cols; ++i) {
            cout << '*';
        }
        cout << endl;
    }
}
