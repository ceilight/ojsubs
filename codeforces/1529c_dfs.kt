import java.io.PrintWriter
import java.io.File
import java.util.StringTokenizer
import kotlin.math.abs
 
fun PrintWriter.solve() {
    repeat(readInt()) {
        var n = readInt()
        var nodes = Array<Array<Int>>(n) { Array<Int>(2) { 0 } }
        for(i in 0..n-1) {
            nodes[i][0] = readInt()
            nodes[i][1] = readInt()
        }
        var adjacent = Array<ArrayList<Int>>(n) { ArrayList<Int>() }
        repeat(n-1) {
            var (u,v) = readInts(2)
            u--; v--
            adjacent[u].add(v)
            adjacent[v].add(u)
        }
        var f = Array<Array<Long>>(n) { Array<Long>(2) { 0L } }
        fun dfs(node: Int, parent: Int) {
            f[node][0] = 0L
            f[node][1] = 0L
            for(child in adjacent[node]) {
                if(child == parent) continue
                dfs(child,node)
                for(i in 0..1) {
                    var maxValue = 0L
                    for(j in 0..1) {
                        maxValue = maxOf(maxValue, f[child][j] + abs(nodes[node][i] - nodes[child][j]).toLong())
                    }
                    f[node][i] += maxValue
                }
            }
        }
        dfs(0,-1)
        var ans = maxOf(f[0][0],f[0][1])
        printf("%d\n",ans)
    }
}

//I/O template 
@JvmField val INPUT = System.`in`
@JvmField val OUTPUT = System.out
 
@JvmField val _reader = INPUT.bufferedReader()
fun readLine(): String? = _reader.readLine()
fun readLn() = _reader.readLine()!!
@JvmField var _tokenizer: StringTokenizer = StringTokenizer("")
fun read(): String {
    while (_tokenizer.hasMoreTokens().not()) _tokenizer = StringTokenizer(_reader.readLine() ?: return "", " ")
    return _tokenizer.nextToken()
}
fun readInt() = read().toInt()
fun readDouble() = read().toDouble()
fun readLong() = read().toLong()
fun readStrings(n: Int) = List(n) { read() }
fun readLines(n: Int) = List(n) { readLn() }
fun readInts(n: Int) = List(n) { read().toInt() }
fun readIntArray(n: Int) = IntArray(n) { read().toInt() }
fun readDoubles(n: Int) = List(n) { read().toDouble() }
fun readDoubleArray(n: Int) = DoubleArray(n) { read().toDouble() }
fun readLongs(n: Int) = List(n) { read().toLong() }
fun readLongArray(n: Int) = LongArray(n) { read().toLong() }
 
@JvmField val _writer = PrintWriter(OUTPUT, false)
inline fun output(block: PrintWriter.() -> Unit) { _writer.apply(block).flush() }
 
fun main(args: Array<String>) { _writer.solve(); _writer.flush() }