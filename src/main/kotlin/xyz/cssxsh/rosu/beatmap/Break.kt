package xyz.cssxsh.rosu.beatmap

import java.nio.ByteBuffer

/**
 * @see Beatmap.breaks
 */
public data class Break(
    val startTime: Double,
    val endTime: Double
) {
    public fun duration(): Double = endTime - startTime

    public companion object {
        @JvmStatic
        @JvmName("fromByteBuffer")
        internal operator fun invoke(buffer: ByteBuffer): Break {
            return Break(
                startTime = buffer.double,
                endTime = buffer.double
            )
        }

        @JvmStatic
        @JvmName("readByteBuffer")
        internal fun sequence(buffer: ByteBuffer): Sequence<Break> = sequence {
            while (buffer.hasRemaining()) {
                yield(invoke(buffer))
            }
        }
    }
}