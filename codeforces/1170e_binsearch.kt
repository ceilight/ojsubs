import java.io.PrintWriter
import java.io.File
import java.util.StringTokenizer
import java.util.TreeSet
 
fun PrintWriter.solve() {
    repeat(1) {
        var (n,m) = readInts(2)
        var a = readIntArray(n)
        var leftSumSet = TreeSet<Int>()
        var sum = 0
        leftSumSet.add(sum)
        for(v in a) {
            sum += v
            leftSumSet.add(sum)
        }
        repeat(readInt()) {
            var size = readInt()
            var usedCpts = readInts(size)
            usedCpts += m+1
            var doorSpace = leftSumSet.lower(usedCpts[0])
            for(i in 0..size-1) {
                var gap = usedCpts[i+1] - usedCpts[i] + doorSpace
                doorSpace = leftSumSet.lower(gap)
            }
            if(doorSpace >= leftSumSet.last()) printf("YES\n"); else printf("NO\n")
        }
    }
}
 
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
 
fun main() { _writer.solve(); _writer.flush() }