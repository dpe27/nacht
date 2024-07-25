package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class EvenSubsetSumProblem_1323A {
    public static void main(String[] args) {
        Reader in = new Reader();
        int t = in.nextInt();
        while (t-- > 0) {
            final int n = in.nextInt();
            int[] arr = new int[n];
            for (int i = 0; i < n; i++) arr[i] = in.nextInt();
            if (n == 1 && arr[0] % 2 != 0) {
                System.out.println(-1);
                continue;
            }
            if (arr[0] % 2 == 0) System.out.printf("1\n%s\n", 1);
            else {
                if (arr[1] % 2 == 0) System.out.printf("1\n%s\n", 2);
                else System.out.printf("2\n%s %s\n", 1, 2);
            }
        }
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
