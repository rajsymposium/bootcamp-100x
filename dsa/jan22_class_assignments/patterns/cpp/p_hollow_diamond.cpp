#include <iostream>
using namespace std;

int main() {
    int rows;
    cin >> rows;

    for (int i = 0; i < rows; ++i) {
        for (int j = 0; j < rows - i - 1; ++j) {
            cout << " ";
        }
        for (int j = 0; j < i + 1; ++j) {
            if (j == 0 || j == i) {
                cout << "* ";
            } else {
                cout << "  ";
            }
        }
        cout << endl;
    }
    for (int i = 1; i < rows; ++i) {
        for (int j = 0; j < i; ++j) {
            cout << " ";
        }
        for (int j = 0; j < rows - i; ++j) {
            if (j == 0 || j == rows - i - 1) {
                cout << "* ";
            } else {
                cout << "  ";
            }
        }
        cout << endl;
    }
}
