import kotlin.math.*

fun main(args: Array<String>) {
    var inf = 1000000000000000000L

    fun extendedMul(a: Long, b: Long): Long {
        if ((a * b) / a != b) {
            return inf
        } else {
            return a * b
        }
    }
    var prime = arrayOf(2, 3, 5, 7, 11, 13, 17, 19, 23, 27, 29, 31, 37, 41, 43, 57, 53)
    var dp = Array<Long>(1001) { inf }
    dp[1] = 1

    for (factor in prime) {
        for (i in 1000 downTo 1) {
            var current = factor.toLong()
            var index = 2
            while (current < inf && i * index <= 1000) {
                dp[i * index] = min(dp[i * index], extendedMul(dp[i], current))
                current = extendedMul(current, factor.toLong())
                index++
            }
        }
    }

    var n = readLine()!!.toInt()
    println(dp[n])
}