package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class GiftCarpet_1862A {
    public static void main(String[] args) {
        Reader in = new Reader();
        int t = in.nextInt();
        while (t-- > 0) {
            int n = in.nextInt();
            int m = in.nextInt();
            char[][] carpet = new char[n][m];
            for (int i = 0; i < n; i++) carpet[i] = in.next().toCharArray();
            solve(carpet, n, m);
        }
    }

    private static void solve(final char[][] carpet, final int n, final int m) {
        final char[] vikaChars = new char[]{'v', 'i', 'k', 'a'};
        int idxVikaChars = 0;
        for (int col = 0 ; col < m ; col++) {
            for (int row = 0 ; row < n ; row++) {
                if (idxVikaChars == vikaChars.length) {
                    System.out.println("YES");
                    return;
                }
                if (carpet[row][col] == vikaChars[idxVikaChars]) {
                    idxVikaChars++;
                    break;
                }
            }
        }
        System.out.println(idxVikaChars == vikaChars.length ? "YES" : "NO");
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
