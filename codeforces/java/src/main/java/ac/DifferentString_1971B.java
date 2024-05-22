package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class DifferentString_1971B {

    public static void main(String[] args) {
        FastReader reader = new FastReader();
        int n = reader.nextInt();
        while (n-- > 0) solve(reader.next());
    }

    private static void solve(String s) {
        for (int i = 1; i < s.length(); i++) {
            if (s.charAt(i-1) != s.charAt(i)) {
                System.out.println("YES");
                System.out.println(s.substring(1) + s.charAt(0));
                return;
            }
        }
        System.out.println("NO");
    }

    private static class FastReader {
        BufferedReader br;
        StringTokenizer st;

        FastReader() {br = new BufferedReader(new InputStreamReader(System.in));}

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
        double nextDouble() {return Double.parseDouble(next());}
        long nextLong() {return Long.parseLong(next());}

        String nextLine() {
            String str = "";
            try {
                if (st.hasMoreTokens()) str = st.nextToken();
                else str = br.readLine();
            } catch (IOException ex) {
                ex.printStackTrace();
            }
            return str;
        }

    }
}
