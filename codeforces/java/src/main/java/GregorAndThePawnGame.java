import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class GregorAndThePawnGame {
    public static void main(String[] args) {
        Reader in = new Reader();
        int t = in.nextInt();
        while (t-- > 0) {
            final int n = in.nextInt();
            final char[] enemy = in.next().toCharArray();
            final char[] gregor = in.next().toCharArray();

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
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
            return st.nextToken();
        }
        int nextInt() {return Integer.parseInt(next());}
    }
}
