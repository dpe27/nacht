package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class KefaAndFirstSteps_580A {

    public static void main(String[] args) {
        Reader in = new Reader();
        int n = in.nextInt();
        int[] arr = new int[n];
        for (int i = 0; i < n; ++i) arr[i] = in.nextInt();
        int maxLength = 1;
        int res = maxLength;

        for (int i = 1; i < n; i++) {
            if (arr[i] >= arr[i-1]) maxLength++;
            else {
                res = Math.max(res, maxLength);
                maxLength = 1;
            }
        }
        res = Math.max(res, maxLength);
        System.out.println(res);
    }

    private static class Reader {
        StringTokenizer st;
        BufferedReader bf;
        Reader() {this.bf = new BufferedReader(new InputStreamReader(System.in));}
        String next() {
            while (st == null || !st.hasMoreElements()) {
                try {
                    st = new StringTokenizer(bf.readLine());
                } catch (IOException ex) {
                    ex.printStackTrace();
                }
            }
            return st.nextToken();
        }
        int nextInt() {return Integer.parseInt(next());}
    }
}
