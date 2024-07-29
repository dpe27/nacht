package ac;

import java.util.*;

// source: https://leetcode.com/problems/longest-substring-without-repeating-characters/
public class LongestSubstringWithoutRepeatingChars {

    private static int solve(String s) {
        int n = s.length();
        int maxLength = 0, start = 0;
        Map<Character, Integer> indexMap = new HashMap<>();

        for (int i = 0; i < n; i++) {
            if (!indexMap.containsKey(s.charAt(i)) || indexMap.get(s.charAt(i)) < start) {
                maxLength = Math.max(maxLength, i - start + 1);
            } else start = indexMap.get(s.charAt(i)) + 1;
            indexMap.put(s.charAt(i), i);
        }

        return maxLength;
    }

    public static void main(String[] args) {
        System.out.println(solve("pwwkew"));
    }
}
