package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class HitTheLottery_996A {
    private static int[] dp;

    public static void main(String[] args) {
        int[] deno = new int[]{100, 20, 10, 5, 1};
        Reader reader = new Reader();
        int n = reader.nextInt();
        int res = 0;
        int i = 0;
        while (n >= 0 && i < deno.length) {
            if (n - deno[i] >= 0) {
                n -= deno[i];
                res++;
            }
            else i++;
        }
        System.out.println(res);
    }



    private static class Reader {
        BufferedReader br;
        StringTokenizer st;

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
