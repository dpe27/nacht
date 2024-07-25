package ac;

import java.util.Arrays;
import java.util.Scanner;

public class ContestProposal_1972A {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int N = Integer.parseInt(sc.nextLine());
        for (int i = 0; i < N; i++) {
            int n = Integer.parseInt(sc.nextLine());
            int[] a = Arrays.stream(sc.nextLine().split(" ")).mapToInt(Integer::parseInt).toArray();
            int[] b = Arrays.stream(sc.nextLine().split(" ")).mapToInt(Integer::parseInt).toArray();
            solve(n, a, b);
        }
    }

    private static void solve(int n, int[] a, int[] b) {
        int diff = 0, res = 0;
        for (int i = 0; i < n; i++) {
            if (a[i - diff] > b[i]) {
               ++diff;
               ++res;
            }
        }
        System.out.println(res);
    }
}
