import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class GiftCarpet_1862A {
    public static void main(String[] args) {
        Reader in = new Reader();
        int t = in.nextInt();
        while (t-- > 0) {

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
