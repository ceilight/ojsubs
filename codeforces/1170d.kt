import java.io.PrintWriter
import java.io.File
import java.util.StringTokenizer
 
fun PrintWriter.solve() {
    repeat(1) {
        var n = readInt()
        var a = readIntArray(n)
        var sequenceNum = a.count{ it==-1 }
        var sequence = Array(sequenceNum) { ArrayList<Int>() }
        var prev = IntArray(sequenceNum)
        var next = IntArray(sequenceNum)
        for(i in 0..sequenceNum-1) {
            prev[i] = (sequenceNum+i-1)%sequenceNum
            next[i] = (i+1)%sequenceNum
        }
        var currentSequence = 0
        for(num in a) {
            if(num == -1) {
                prev[next[currentSequence]]=prev[currentSequence]
                next[prev[currentSequence]]=next[currentSequence]
            }
            else {
                sequence[currentSequence].add(num)
            }
            currentSequence = next[currentSequence]
        }
        printf("%d\n",sequenceNum);
        for(i in 0..sequenceNum-1) {
            printf("%d ",sequence[i].size)
            for(v in sequence[i]) {
                printf("%d ",v);
            }
            printf("\n");
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