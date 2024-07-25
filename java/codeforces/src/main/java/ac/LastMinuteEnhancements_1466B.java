package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Arrays;
import java.util.StringTokenizer;

public class LastMinuteEnhancements_1466B {
    public static void main(String[] args) {
        Reader reader = new Reader();
        int t = reader.nextInt();
        while (t-- > 0) {
            final int n = reader.nextInt();
            int[] arr = new int[n];
            for (int i = 0; i < n; ++i) arr[i] = reader.nextInt();
            int [] dp = new int[arr[n - 1] + 2];
            Arrays.fill(dp, 0);
            for (int i = 0; i < n; ++i) dp[arr[i]]++;
            int res = 0;
            for (int i = dp.length - 1; i > 1; --i) {
                if (dp[i] == 0 && dp[i-1] != 0) {
                    dp[i]++;
                    dp[i-1]--;
                }
            }
            for (int i = 1; i < dp.length; ++i) if (dp[i] != 0) res++;
            System.out.println(res);
        }
    }

    private static class Reader {
        StringTokenizer st;
        BufferedReader br;

        Reader() {this.br = new BufferedReader(new InputStreamReader(System.in));}
        String next() {
            while (st == null || !st.hasMoreElements()) {
                try {
                    st = new StringTokenizer(br.readLine());
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
            return st.nextToken();
        }
        int nextInt() {return Integer.parseInt(next());}
    }
}
