package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class FairDivision_1472B {
    public static void main(String[] args) {
        Reader reader = new Reader();
        int t = reader.nextInt();
        while (t-- > 0) {
            int n = reader.nextInt();
            int cnt1 = 0;
            for (int i = 0 ; i < n ; i++) if (reader.nextInt() == 1) cnt1++;
            int cnt2 = n - cnt1;
            if ((cnt1 + 2 * cnt2) % 2 != 0) System.out.println("NO");
            else {
                int sum = (cnt1 + 2 * cnt2) / 2;
                if (sum % 2 == 0 || (sum % 2 == 1 && cnt1 != 0)) System.out.println("YES");
                else System.out.println("NO");
            }
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
                } catch (IOException ex) {
                    ex.printStackTrace();
                }
            }
            return st.nextToken();
        }
        int nextInt() {return Integer.parseInt(next());}
    }
}
