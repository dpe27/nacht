package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class AlexAndARhombus_1180A {
    public static void main(String[] args) {
        Reader in = new Reader();
        final int n = in.nextInt();
        int[] dp = new int[n];
        dp[0] = 1;
        for (int i = 1; i < n; i++) dp[i] = dp[i-1] + 4 * i;
        System.out.println(dp[n-1]);
    }

    private static class Reader {
        StringTokenizer st;
        BufferedReader br;
        Reader() {br = new BufferedReader(new InputStreamReader(System.in));}
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
