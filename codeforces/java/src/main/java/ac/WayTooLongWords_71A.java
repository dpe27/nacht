package ac;

import java.util.Scanner;
// [DONE!!!]
public class WayTooLongWords_71A {

    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = Integer.parseInt(sc.nextLine());
        for (int i = 0; i < n; i++) {
            String str = sc.nextLine();
            if (str.length() > 10) {
                System.out.printf("%s%s%s\n", str.charAt(0), str.length() - 2, str.charAt(str.length() - 1));
                return;
            }
            System.out.println(str);
        }
    }
}
