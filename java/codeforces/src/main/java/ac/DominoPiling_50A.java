package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class DominoPiling_50A {
    public static void main(String[] args) {
        Reader reader = new Reader();
        final int m = reader.nextInt();
        final int n = reader.nextInt();
        System.out.println(n / 2 * m + (n % 2 != 0 ? (m / 2) : 0));
    }

    private static class Reader {
        StringTokenizer st;
        BufferedReader br;

        Reader() {this.br = new BufferedReader(new InputStreamReader(System.in));}
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
