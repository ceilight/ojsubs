import java.io.PrintWriter
import java.io.File
import java.util.StringTokenizer
 
fun PrintWriter.solve() {
    var dir = mutableListOf<String>()
    repeat(readInt()) {
        var cmd = read()
        if(cmd == "cd") {
            var path = read()
            var idx = 0
            var toDir = ""
            if(path[idx] == '/') {
                dir.clear()
                idx++
            }
            while(idx < path.length) {
                if(path[idx] == '/' && toDir.length > 0) {
                    dir.add(toDir)
                    toDir = ""
                    idx++
                    continue
                }
                else if(path[idx] == '.') {
                    dir.remove(dir.last())
                    if(idx + 3 < path.length)
                        idx += 3
                    else
                        idx += 2
                    continue
                }
                toDir += path[idx]
                idx++
            }
            if(toDir.length > 0) {
                dir.add(toDir)
                toDir = ""
            }
        }
        else {
            if(dir.size > 0)
                for(par in dir)
                    printf("/%s", par)
            printf("/\n")
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