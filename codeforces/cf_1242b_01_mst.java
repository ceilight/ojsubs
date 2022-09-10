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

import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.TreeSet;
import java.util.Map;
import java.util.HashMap;

public class sandbox {
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
        static int[] parent = new int[100100];
        static int[] extent = new int[100100];
        static List<Integer>[] edge = new ArrayList[100100];
     
        public void solve(int ttest, InputReader in, PrintWriter out) {
            int n = in.nextInt();
            int m = in.nextInt();
            for(int i = 0; i < n; i++) {
                edge[i] = new ArrayList<>();
            }
            for(int i = 0; i < m; i++) {
                int u = in.nextInt() - 1;
                int v = in.nextInt() - 1;
                edge[u].add(v);
                edge[v].add(u);
            }
            Set<Integer> comp = new TreeSet<>();
            for(int i = 0; i < n; i++) {
                parent[i] = i;
                extent[i] = 1;
                comp.add(i);
            }
            for(int i = 0; i < n; i++) {
                Map<Integer, Integer> cnt = new HashMap();
                for(int j : comp) {
                    cnt.put(j, 0);
                }
                for(int j : edge[i]) {
                    cnt.put(find(j), cnt.get(find(j)) + 1);
                }
                Set<Integer> newComp = new TreeSet<>();
                for(int j : comp) {
                    if(cnt.get(j) == extent[j]) {
                        newComp.add(j);
                    } else {
                        unite(i, j);
                    }
                }
                newComp.add(find(i));
                comp = newComp;
            }
            out.println(comp.size() - 1);
        }
     
        public static int find(int a) {
            if(a == parent[a]) return a;
            parent[a] = find(parent[a]);
            return parent[a];
        }
        
        public static boolean unite(int u, int v) {
            int i = find(u);
            int j = find(v);
            if(i == j) return false;
            extent[i] += extent[j];
            parent[j] = i;
            return true;
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