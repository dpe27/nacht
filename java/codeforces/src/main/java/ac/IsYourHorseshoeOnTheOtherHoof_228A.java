package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Arrays;
import java.util.StringTokenizer;

public class IsYourHorseshoeOnTheOtherHoof_228A  {
    public static void main(String[] args) {
        FastReader reader = new FastReader();
        int[] arr = new int[4];
        for (int i = 0; i < 4; i++) arr[i] = reader.nextInt();
        Arrays.sort(arr);
        int res = 0;
        for (int i = 1; i < 4; i++) if (arr[i] == arr[i-1]) res += 1;
        System.out.println(res);
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
