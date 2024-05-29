package tle;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class LoveSong_1539B {
    public static void main(String[] args) {
        Reader in = new Reader();
        final int n = in.nextInt();
        int q = in.nextInt();
        final char[] s = in.next().toCharArray();
        int[] dp = new int[26];
        while (q-- > 0) {
            int l = in.nextInt() - 1;
            int r = in.nextInt() - 1;
            int res = 0;
            for (int i = l; i <= r; i++) res += s[i] - 'a' + 1;
            System.out.println(res);
        }

    }

    private static class Reader {
        StringTokenizer st;
        BufferedReader br;
        Reader() {br = new BufferedReader(new InputStreamReader(System.in));}
        String next() {
            while (st == null || !st.hasMoreElements()) {
                try {
                    st = new StringTokenizer(br.readLine());
                } catch (IOException ex) {
                    ex.printStackTrace();
                }
            }
            return st.nextToken();
        }
        int nextInt() {return Integer.parseInt(next());}
    }
}
