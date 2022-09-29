import java.io.PrintWriter
import java.io.File
import java.util.StringTokenizer
import kotlin.Comparator

data class Edge(var u: Int, var v: Int, var weight: Long, var id: Int)

var graph = Array<MutableList<Int>>(200100) { ArrayList<Int>() }
var depth = Array<Int>(200100) { 0 }
var edges: MutableList<Edge> = ArrayList<Edge>()
var edgesMST: MutableList<Edge> = ArrayList<Edge>()

fun PrintWriter.solve() {
    var (n, m) = readInts(2)
    for (i in 0..m-1) {
        var (u, v) = readInts(2)
        var w = readLong()
        edges.add(Edge(u, v, w, i))
    }

    var mstWeight = kruskal(n)
    // println(mstWeight)
    var solver = Solver(n)
    var ans = LongArray(m) { 0 }
    for (e in edges) {
        ans[e.id] = mstWeight - solver.getMaxEdge(e.u, e.v) + e.weight
    }
    println(ans.joinToString("\n"))
}

class UnionFind(n: Int) {
    var setSize = IntArray(n+1) { 1 }
    var setRoot = IntArray(n+1) { it }

    fun findSet(u: Int): Int {
        if (setRoot[u] == u) return u
        setRoot[u] = findSet(setRoot[u])
        return setRoot[u]
    }

    fun uniteSets(u: Int, v: Int): Boolean {
        var ru = findSet(u)
        var rv = findSet(v)
        if (ru == rv) return false
        if (setSize[ru] < setSize[rv]) {
            ru = rv.also { rv = ru }
        }
        setSize[ru] += setSize[rv]
        setRoot[rv] = ru
        return true
    }
}

fun kruskal(n: Int): Long {
    edges.sortWith(Comparator { e1, e2 -> (e1.weight - e2.weight).toInt() })
    var totWeight: Long = 0L

    var dsu: UnionFind = UnionFind(n)
    for (e in edges) {
        if (edgesMST.size == n-1) break
        if (dsu.uniteSets(e.u, e.v)) {
            edgesMST.add(e)
            totWeight += e.weight
            graph[e.u].add(e.v)
            graph[e.v].add(e.u)
        }
    }
    return totWeight
}

class Solver(n: Int) {
    var ancestor = Array<IntArray>(n+1) {
        IntArray(20) { 0 }
    } 
    var maxEdge = Array<LongArray>(n+1) {
        LongArray(20) { 0L }
    }
    init {
        dfs(1, 0, 0)
        for (e in edgesMST) {
            if (depth[e.u] < depth[e.v]) {
                e.u = e.v.also { e.v = e.u }
            }
            ancestor[e.u][0] = e.v
            maxEdge[e.u][0] = e.weight
        }
        for (k in 1..19) {
            for (i in 1..n) {
                ancestor[i][k] = ancestor[ancestor[i][k-1]][k-1]
                maxEdge[i][k] = maxOf(maxEdge[ancestor[i][k-1]][k-1], maxEdge[i][k-1])
            }
        }
    }

    fun getMaxEdge(st: Int, ed: Int): Long {
        var u = st
        var v = ed
        if (u == ancestor[v][0]) return maxEdge[v][0]
        if (v == ancestor[u][0]) return maxEdge[u][0]
 
        if (depth[u] < depth[v]) u = v.also {v = u}
 
        var res = 0L
        while (depth[u] > depth[v]) {
            for (i in 19 downTo 0) {
                if (depth[u] - (1 shl i) >= depth[v]) {
                    res = maxOf(res, maxEdge[u][i])
                    u = ancestor[u][i]
                    break
                }
            }
        }
        if (u == v) return res
 
        for (i in 19 downTo 0) {
            if (u == v) break
            if (ancestor[u][i] != ancestor[v][i]) {
                res = maxOf(res, maxEdge[u][i], maxEdge[v][i])
                u = ancestor[u][i]
                v = ancestor[v][i]
            }
        }
        return maxOf(res, maxEdge[u][0], maxEdge[v][0])
    }

    fun dfs(u: Int, par: Int, curDepth: Int) {
        depth[u] = curDepth
        for (v in graph[u]) {
            if (v != par) {
                dfs(v, u, curDepth+1)
            }
        }
    }
}




@JvmField
val INPUT = System.`in`
@JvmField
val OUTPUT = System.out

@JvmField
val _reader = INPUT.bufferedReader()
fun readLine(): String? = _reader.readLine()
fun readLn() = _reader.readLine()!!
@JvmField
var _tokenizer: StringTokenizer = StringTokenizer("")
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
 
@JvmField
val _writer = PrintWriter(OUTPUT, false)
inline fun output(block: PrintWriter.() -> Unit) { _writer.apply(block).flush() }
 
fun main(args: Array<String>) { _writer.solve(); _writer.flush() }