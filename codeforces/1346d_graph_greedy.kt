import java.io.PrintWriter
import java.io.File
import java.util.StringTokenizer
 
fun PrintWriter.solve() {
    data class Edge(
        var left: Int,
        var right: Int,
        var weight: Int
    )
    repeat(readInt()) {
        var (n,m) = readInts(2)
        var tunnels = Array<Edge>(m) {
            var (u,v,w) = readInts(3)
            Edge(u-1,v-1,w)
        }.sortedByDescending { it.weight }
        var monsterCount = IntArray(n) { 0 }
        var check: Boolean = true
        for(t in tunnels) {
            if(monsterCount[t.left] == 0) {
                monsterCount[t.left] = t.weight
            }
            if(monsterCount[t.right] == 0) {
                monsterCount[t.right] = t.weight
            }
            if(t.weight != minOf(monsterCount[t.left],monsterCount[t.right])) {
                check = false
                break
            }
        }
        if(check) {
            printf("YES\n")
            for(i in monsterCount) {
                printf("%d ",i)
            }
            printf("\n")
        }
        else {
            printf("NO\n")
        }
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