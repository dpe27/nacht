// C. Sum in Binary Tree [COOK LUON]
// Source: https://codeforces.com/problemset/problem/1843/C

#include <bits/stdc++.h>
using namespace std;

typedef long long ll;

int solve(ll n) {
    ll sum = 0;
    while (n >= 1) {
        sum += n;
        n /= 2;
    }
    return sum;
}

int main(int argc, char const *argv[])
{
    int tests;
    cin >> tests;
    for (int i = 0; i < tests;++i) {
        ll n; cin >> n;
        cout << solve(n) << endl;
    }
    return 0;
}
