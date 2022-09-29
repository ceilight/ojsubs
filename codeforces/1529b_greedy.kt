import java.io.PrintWriter
import java.io.File
import java.util.StringTokenizer
 
fun PrintWriter.solve() {
    for(i in 1..readInt()) {
        var n = readInt()
        var a = readInts(n).sorted()
        var nonPos = a.count { it <= 0 }
        if(nonPos == n) {
            printf("%d\n",n)
            continue
        }
        var minPosVal = a[nonPos]
        var check: Boolean = true
        for(i in 1..nonPos - 1) {
            if(a[i] - a[i-1] < minPosVal) {
                check = false
            }
        }
        printf("%d\n", if(check) nonPos+1; else nonPos)
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