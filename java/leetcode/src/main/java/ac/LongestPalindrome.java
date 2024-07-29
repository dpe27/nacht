package ac;

import java.util.Arrays;

public class LongestPalindrome {

    // Dynamic programming
    // Time complexity: O(N2)
    // Space complexity: O(N2)
    private static String solve(String s) {
        char[] str = s.toCharArray();
        int n = str.length;
        boolean[][] dp = new boolean[n][n];
        for (boolean[] row : dp) Arrays.fill(row, false);

        int maxLen = 1;
        for (int i = 0; i < n; i++) dp[i][i] = true;

        // checking for sub-string of length 2
        int start = 0;
        for (int i = 0; i < n - 1; i++) {
            if (str[i] == str[i+1]) {
                dp[i][i+1] = true;
                start = i;
                maxLen = 2;
            }
        }

        // checking for sub-string of length greater than 2
        // k is length of sub-string
        for (int k = 3; k <= n; k++) {
            for (int i = 0; i < n-k+1; i++) {
                int j = i + k - 1;
                if (dp[i+1][j-1] && str[i] == str[j]) {
                    dp[i][j] = true;
                    if (k > maxLen) {
                        maxLen = k;
                        start = i;
                    }
                }
            }
        }
        return s.substring(start, start + maxLen);
    }

    // Time complexity: O(N2)
    // Space complexity: O(1)
    private static String solve2(String s) {
        char[] str = s.toCharArray();
        int n = str.length;
        int start = 0, maxLen = 1;
        int low, high;

        for (int i = 0; i < n; i++) {
            // find longest palindromic substring of even size
            low = i - 1;
            high = i;
            while (low >= 0 && high < n && str[low] == str[high]) {
                if (high - low + 1 > maxLen) {
                    maxLen = high - low + 1;
                    start = low;
                }
                low--;
                high++;
            }

            // find longest palindromic substring of odd size
            low = i - 1;
            high = i + 1;
            while (low >= 0 && high < n && str[low] == str[high]) {
                if (high - low + 1 > maxLen) {
                    maxLen = high - low + 1;
                    start = low;
                }
                low--;
                high--;
            }
        }
        return s.substring(start, start + maxLen);
    }

    public static void main(String[] args) {
        System.out.println(solve("forgeeksskeegfor"));
    }
}
