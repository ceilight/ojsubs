import java.io.PrintWriter
import java.io.File
import java.util.StringTokenizer
import kotlin.math.abs
 
fun PrintWriter.solve() {
    repeat(1) {
        var n = readInt()
        var a = readInts(n)
        var ones = Array<Int>(20) { 0 }
        for(i in 0..n-1) for(j in 0..19) {
            if((a[i] shr j) and 1 == 1) {
                ones[j]++
            }
        }
        var sum: Long = 0L
        for(i in 0..n-1) {
            var value: Long = 0L
            for(j in 0..19) {
                if(i < ones[j]) {
                    value = value or (1L shl j)
                }
            }
            sum += value * value
        }
        printf("%d\n", sum)
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