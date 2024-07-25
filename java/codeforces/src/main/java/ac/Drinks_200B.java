package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class Drinks_200B {
    public static void main(String[] args) {
        FastReader reader = new FastReader();
        int n = reader.nextInt();
        double res = 0;
        for (int i = 0; i < n; i++) res += reader.nextDouble() / n;
        System.out.printf("%.12f", res);
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
        float nextFloat() {return Float.parseFloat(next());}
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
