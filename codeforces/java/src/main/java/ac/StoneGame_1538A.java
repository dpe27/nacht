package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class StoneGame_1538A {
    public static void main(String[] args) {
        Reader reader = new Reader();
        int t = reader.nextInt();
        while (t-- > 0) {
            int n = reader.nextInt();
            int iMax = 0;
            int iMin = 0;
            int[] arr = new int[n];
            for (int i = 0; i < n; i++) arr[i] = reader.nextInt();
            int currMax = arr[0];
            int currMin = arr[0];
            for (int i = 1; i < n; i++) {
                if (arr[i] > currMax) {
                    currMax = arr[i];
                    iMax = i;
                }
                if (arr[i] < currMin) {
                    currMin = arr[i];
                    iMin = i;
                }
            }
            int x = Math.max(iMin + 1, iMax + 1);
            int y = Math.max(n - iMin, n - iMax);
            int z = Math.min(iMin + 1, n - iMin) + Math.min(iMax + 1, n - iMax);
            System.out.println(Math.min(Math.min(x, y), z));
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