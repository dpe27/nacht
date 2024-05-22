package ac;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.List;
import java.util.StringTokenizer;

public class QAQ_894A {
    public static void main(String[] args) {
        Reader in = new Reader();
        char[] inputStr = in.next().toCharArray();
        if (inputStr.length <= 2) {
            System.out.println(0);
            return;
        }
        List<Integer> dp = new ArrayList<>();
        int ACharCount = 0;
        boolean flag = false;
        for (char c : inputStr) {
            if (c == 'Q') {
                if (flag) dp.add(ACharCount);
                ACharCount = 0;
                flag = true;
            } else if (c == 'A') ACharCount++;
        }
        if (dp.isEmpty()) {
            System.out.println(0);
            return;
        }
        int res = 0;
        for (int j = dp.size() - 2; j >= 0; j--) {
            dp.set(j, dp.get(j+1) +  (dp.size() - j) * dp.get(j));
            res += dp.get(j);
        }

        res += dp.getLast();
        System.out.println(res);
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
    }
}

