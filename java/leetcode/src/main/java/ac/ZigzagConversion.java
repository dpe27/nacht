package ac;

import java.util.ArrayList;
import java.util.List;

public class ZigzagConversion {

    private static String convert(String s, int numRows) {
        if (numRows <= 1) return s;
        char[] carr = s.toCharArray();
        List<StringBuilder> rowList = new ArrayList<>(numRows);
        for (int i = 0; i < numRows; i++) rowList.add(new StringBuilder());

        int rowIndex = 0;
        boolean direction = true;
        for (char c : carr) {
            if (rowIndex == 0 || rowIndex == numRows - 1) direction = !direction;
            rowList.get(rowIndex).append(c);
            if (!direction) rowIndex++;
            else rowIndex--;
        }
        return String.join("", rowList.stream().map(StringBuilder::toString).toList());
    }

    public static void main(String[] args) {
        System.out.println(convert("PAYPALISHIRING", 4).equals("PINALSIGYAHRPI"));

    }
}
