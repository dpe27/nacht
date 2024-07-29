package tmp;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.*;

public class KefaAndPark_580C {

    public static void main(String[] args) {
        Reader in = new Reader();
        int n = in.nextInt();
        int m = in.nextInt();
        int[] verCatArr = new int[n];
        for (int i = 0; i < n; i++) verCatArr[i] = in.nextInt();

        Map<Integer, Set<Integer>> edges = new HashMap<>(n);
        for (int i = 0; i < n-1; i++) {
            int x = in.nextInt() - 1;
            int y = in.nextInt() - 1;
            if (edges.containsKey(x)) edges.get(x).add(y);
            else {
                Set<Integer> adjVertices = new HashSet<>();
                adjVertices.add(y);
                edges.put(x, adjVertices);
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
