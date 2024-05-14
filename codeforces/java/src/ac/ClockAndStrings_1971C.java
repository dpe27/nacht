package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class ClockAndStrings_1971C {
    public static void main(String[] args) {
        FastReader reader = new FastReader();
        int n = reader.nextInt();
        while (n-- > 0) {
            int[] arr = new int[4];
            for (int i = 0; i < 4; ++i) arr[i] = reader.nextInt();
            int min1 = Math.min(arr[0], arr[1]);
            int max1 = Math.max(arr[0], arr[1]);
            int min2 = Math.min(arr[2], arr[3]);
            int max2 = Math.max(arr[2], arr[3]);

            boolean check = (min1 > min2 && min1 < max2 && max1 > max2) || (min2 > min1 && min2 < max1 && max2 > max1);
            System.out.println(check ? "YES" : "NO");
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
