package xyz.cssxsh.osu.beatmap

/**
 * @see Beatmap.breaks
 */
public data class Break(
    val startTime: Double,
    val endTime: Double
) {
    public fun duration(): Double = endTime - startTime
}