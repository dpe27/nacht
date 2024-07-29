package ac;

public class ReverseInteger {
    private static int reverse(int x) {
        int r = 0;
        while (x != 0) {
            if (r < Integer.MIN_VALUE/10 || r > Integer.MAX_VALUE/10) return 0;
            r = r * 10 + x % 10;
            x /= 10;
        }
        return r;
    }

    public static void main(String[] args) {

        System.out.println();
    }
}
