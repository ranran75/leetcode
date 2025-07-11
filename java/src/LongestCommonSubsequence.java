//https://leetcode.cn/problems/longest-common-subsequence/description/
public class LongestCommonSubsequence {

    public static void main(String[] args) {
        String a = "book_readaa3";
        String b = "adfljbookasdflkreada3";
        System.out.println(longestCommonSubsequence(a,b));
    }

    public static int longestCommonSubsequence(String stringA, String stringB) {
        char[] a = stringA.toCharArray();
        char[] b = stringB.toCharArray();
        int[][] dp = new int[a.length + 1][b.length + 1];

        for (int i = 1; i <= a.length; i++) {
            for (int j = 1; j <= b.length; j++) {
                if (a[i-1] == b[j-1]){
                    dp[i][j] = dp[i-1][j-1] + 1;
                }
                else {
                    dp[i][j] = Math.max(dp[i-1][j], dp[i][j-1]) ;
                }
            }
        }
        return dp[a.length ][b.length];
    }
}
