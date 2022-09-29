import java.io.PrintWriter
import java.io.File
import java.util.StringTokenizer
import kotlin.math.abs

fun PrintWriter.solve() {
    data class Edge (
        var startNode: Int, 
        var endNode: Int, 
        var weight: Int
    )
    repeat(readInt()) {
        var (n,m,k) = readInts(3)
        var rank = IntArray(n) { 0 }
        var root = IntArray(n)
        for(i in 0..n-1) {
            root[i] = i
        }
        var edges = Array<Edge>(m) {
            var (x,y,s) = readInts(3)
            Edge(x-1,y-1,s)
        }.sortedBy{ it.weight }
        var changeCount: Long = 0L
        for(i in 0..m-1) {
            var start = edges[i].startNode
            var end = edges[i].endNode
            while(root[start] != start) {
                start = root[start]
            }
            while(root[end] != end) {
                end = root[end]
            }
            if(start == end) continue
            if(rank[start] < rank[end]) {
                start = end.also { end = start }
            }
            else if(rank[start] == rank[end]) {
                rank[start]++
            }
            root[end] = start
            changeCount += maxOf(0L, (edges[i].weight - k).toLong())
        }
        if(changeCount == 0L) {
            changeCount = 1000000007L
            for(i in 0..m-1) {
                changeCount = minOf(changeCount, abs(edges[i].weight - k).toLong())
            }
        }
        printf("%d\n",changeCount)
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