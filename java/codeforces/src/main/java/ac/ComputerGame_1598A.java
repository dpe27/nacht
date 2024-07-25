package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

public class ComputerGame_1598A {
    public static void main(String[] args) {
        Reader reader = new Reader();
        int t = reader.nextInt();
        while (t-- > 0) {
            final int n = reader.nextInt();
            char[][] grid = new char[2][n];
            for (int i = 0; i < 2; i++) grid[i] = reader.next().toCharArray();

            boolean flag = false;
            for (int i = 0; i < n; i++) flag |= grid[0][i] == '1' && grid[1][i] == '1';
            System.out.println(flag ? "NO" : "YES");
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
