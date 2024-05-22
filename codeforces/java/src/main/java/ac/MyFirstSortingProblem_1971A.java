package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class MyFirstSortingProblem_1971A {

    public static void main(String[] args) {
        FastReader reader = new FastReader();
        int n = reader.nextInt();
        while (n-- > 0) {
            int[] arr = new int[2];
            for (int i = 0; i < 2; i++) arr[i] = reader.nextInt();
            System.out.printf("%s %s\n", Math.min(arr[0], arr[1]), Math.max(arr[0], arr[1]));
        }
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
