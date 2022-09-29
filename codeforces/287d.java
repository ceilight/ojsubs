import java.io.OutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.PrintWriter;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;
import java.io.BufferedReader;
import java.util.Comparator;
import java.io.InputStream;

public class 287d {
    public static void main(String[] args) {
        InputStream inputStream = System.in;
        OutputStream outputStream = System.out;
        InputReader in = new InputReader(inputStream);
        PrintWriter out = new PrintWriter(outputStream);
        task solver = new task();
        solver.solve(1, in, out);
        out.close();
    }

    static class task {
        static int[] perm;

        public void solve(int ttest, InputReader in, PrintWriter out) {
            int n = in.nextInt();
            perm = new int[n * 2 + 3];
            for(int i = 1; i <= n; i++) {
                perm[i] = i;
            }
            for(int i = 2; i <= n; i++) {
                int cur = 0;
                for(int j = i - 1; j < n + i - 1; j += i) {
                    int temp = cur;
                    cur = perm[j];
                    perm[j] = temp;
                }
                perm[n + i - 1] = cur;
            }
            for(int i = n; i < n * 2; i++) {
                out.printf("%d ", perm[i]);
            }
        }
    }

    static class InputReader {
        public BufferedReader reader;
        public StringTokenizer tokenizer;

        public InputReader(InputStream stream) {
            reader = new BufferedReader(new InputStreamReader(stream), 32768);
            tokenizer = null;
        }

        public String next() {
            while (tokenizer == null || !tokenizer.hasMoreTokens()) {
                try {
                    tokenizer = new StringTokenizer(reader.readLine());
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
            }
            return tokenizer.nextToken();
        }

        public int nextInt() {
            return Integer.parseInt(next());
        }
    }
}