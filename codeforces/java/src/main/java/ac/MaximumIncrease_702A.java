package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.*;

public class MaximumIncrease_702A {
    public static void main(String[] args) {
        Reader in = new Reader();
        int n = in.nextInt();
        int k = 1;
        int[] arr = new int[n];
        List<Integer> dp = new ArrayList<>();
        for (int i = 0; i < n; i++) arr[i] = in.nextInt();
        for (int i = 1; i < n; i++) {
            if (arr[i] > arr[i - 1]) ++k;
            else {
                dp.add(k);
                k = 1;
            }
        }
        dp.add(k);
        System.out.println(Collections.max(dp));
    }

    private static class Reader {
        StringTokenizer st;
        BufferedReader br;
        Reader() {br = new BufferedReader(new InputStreamReader(System.in));}
        String next()  {
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
