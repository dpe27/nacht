package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class LastYearSubstring_1462B {

    public static void main(String[] args) {
        Reader in = new Reader();
        int t = in.nextInt();
        while (t-- > 0) {
            final int n = in.nextInt();
            final String s = in.next();
            if (n < 4) {
                System.out.println("NO");
                continue;
            }
            System.out.println(
                    s.startsWith("2020") || s.endsWith("2020") ||
                    (s.startsWith("202") && s.endsWith("0")) ||
                    (s.startsWith("2") && s.endsWith("020")) ||
                    (s.startsWith("20") && s.endsWith("20"))
                    ? "YES" : "NO"
            );
        }
    }


    private static class Reader {
        StringTokenizer st;
        BufferedReader br;
        public Reader() {br = new BufferedReader(new InputStreamReader(System.in));}
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
