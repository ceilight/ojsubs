import java.io.PrintWriter
import java.io.File
import java.util.StringTokenizer
 
fun PrintWriter.solve() {
    repeat(readInt()) {
        var (n,k1,k2) = readInts(3)
        var s = read()
        var a = IntArray(n) { 0 }
        a[0] = if(s[0] == '1') k1; else 0
        var sum = a[0]
        for(i in 1..n-1) {
            if(s[i] == '1' && s[i-1] == '1') {
                a[i] = minOf(k1, k2 - a[i-1])
            }
            else if(s[i] == '1' && s[i-1] == '0') {
                a[i] = k1
            }
            sum += a[i]
        }
        printf("%d\n",sum)
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