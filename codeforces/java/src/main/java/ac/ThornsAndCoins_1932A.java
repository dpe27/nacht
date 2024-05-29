package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class ThornsAndCoins_1932A {
    public static void main(String[] args) {
        Reader in = new Reader();
        int t = in.nextInt();
        while (t-- > 0) {
            final int n = in.nextInt();
            final char[] path = in.next().toCharArray();
            int res = 0;
            for (int i = 1; i < path.length; i++) {
                res += (path[i] == '@') ? 1 : 0;
                if (path[i] == '*' && path[i - 1] == '*') break;
            }
            System.out.println(res);
        }
    }

    private static class Reader {
        BufferedReader br;
        StringTokenizer st;
        Reader() {br = new BufferedReader(new InputStreamReader(System.in)); }
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
