package tmp;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class MinMexCut_1566B {
    public static void main(String[] args) {
        Reader in = new Reader();
        int t = in.nextInt();
        while (t-- > 0) {

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
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
            return st.nextToken();
        }
        int nextInt() {return Integer.parseInt(next());}
    }
}
