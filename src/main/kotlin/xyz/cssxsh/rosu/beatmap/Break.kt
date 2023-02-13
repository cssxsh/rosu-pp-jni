package xyz.cssxsh.rosu.beatmap

import java.nio.ByteBuffer
import java.nio.ByteOrder

/**
 * @see Beatmap.breaks
 */
public data class Break(
    val startTime: Double,
    val endTime: Double
) {
    public fun duration(): Double = endTime - startTime

    public companion object Native {
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
        internal fun sequence(
            buffer: ByteBuffer,
            order: ByteOrder = ByteOrder.LITTLE_ENDIAN
        ): Sequence<Break> = sequence {
            buffer.order(order)
            while (buffer.hasRemaining()) {
                yield(invoke(buffer))
            }
        }
    }
}