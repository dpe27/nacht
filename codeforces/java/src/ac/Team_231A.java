package ac;

import java.util.Arrays;
import java.util.Scanner;
// DONE !!!
public class Team_231A {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = Integer.parseInt(sc.nextLine());
        int res = 0;
        for (int i = 0; i < n; i++) {
            int sum = Arrays.stream(sc.nextLine().split(" "))
                    .mapToInt(Integer::parseInt)
                    .sum();
            if (sum >= 2) res++;
        }
        System.out.println(res);
    }
}
