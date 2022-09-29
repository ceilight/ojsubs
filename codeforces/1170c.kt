import java.io.PrintWriter
import java.io.File
import java.util.StringTokenizer
 
fun PrintWriter.solve() {
    repeat(readInt()) {
        var init = read()
        var target = read()
        var (i,t) = Pair(0,0)
        while(i < init.length && t < target.length) {
            if(init[i] == '+' && target[t] == '-') break
            else if(init[i] == '-' && target[t] == '+') {
                if(i+1 < init.length && init[i+1] == '-') {
                    i+=2; t++
                }
                else break
            }
            else {
                i++; t++;
            }
        }
        if(i == init.length && t == target.length) printf("YES\n"); else printf("NO\n")
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