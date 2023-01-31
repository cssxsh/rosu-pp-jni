package xyz.cssxsh.osu.rust

/**
 * @see Beatmap.breaks
 */
public data class Break(
    val startTime: Float,
    val endTime: Float
) {
    public fun duration(): Float = endTime - startTime
}