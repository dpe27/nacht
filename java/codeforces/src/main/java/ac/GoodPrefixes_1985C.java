package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class GoodPrefixes_1985C {
    public static void main(String[] args) {
        Reader in = new Reader();
        int t = in.nextInt();
        while (t-- > 0) {
            final int n = in.nextInt();
            int[] arr = new int[n];
            for (int i = 0; i < n; i++) arr[i] = in.nextInt();
            long sum = 0;
            int maxEle = 0, res = 0;
            for (int i = 0; i < n; i++) {
                sum += arr[i];
                maxEle = Math.max(maxEle, arr[i]);
                if ((long) maxEle * 2 == sum) res++;
            }
            System.out.println(res);
        }
    }

    private static class Reader {
        StringTokenizer st;
        BufferedReader br;
        Reader() {this.br = new BufferedReader(new InputStreamReader(System.in));}
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
