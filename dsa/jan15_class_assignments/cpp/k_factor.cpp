#include <iostream>
using namespace std;

int main()
{
	int n;
	int f;
	cin >> n >> f;

	if (n % f == 0) {
		cout << "Yes" << endl;
	} else {
		cout << "No" << endl;
	}
}
