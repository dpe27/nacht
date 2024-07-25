package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Arrays;
import java.util.Comparator;
import java.util.StringTokenizer;

public class BalancedShuffle_1970A1 {
    public static void main(String[] args) {
        FastReader reader = new FastReader();
        char[] paSe = reader.next().toCharArray();
        int b = 0;
        StringBuffer res = new StringBuffer(paSe.length);
        int[][] arr = new int[paSe.length][3];
        for (int i = 0; i < paSe.length; i++) {
            arr[i] = new int[]{b, -i, i};
            if (paSe[i] == '(') ++b;
            else --b;
        }
        Arrays.sort(arr, Comparator.comparingInt((int[] ele) -> ele[0]).thenComparingInt(ele -> ele[1]));
        for (int i = 0; i < paSe.length; ++i) res.append(paSe[arr[i][2]]);
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
