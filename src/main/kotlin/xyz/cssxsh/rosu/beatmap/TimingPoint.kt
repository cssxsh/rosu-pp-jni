package xyz.cssxsh.rosu.beatmap

import java.nio.ByteBuffer
import java.nio.ByteOrder

public data class TimingPoint(
    val beatLen: Double,
    val time: Double
) {
    public companion object Native {
        @JvmStatic
        @JvmName("fromByteBuffer")
        internal operator fun invoke(buffer: ByteBuffer, order: ByteOrder = ByteOrder.LITTLE_ENDIAN): TimingPoint {
            buffer.order(order)
            return TimingPoint(
                beatLen = buffer.double,
                time = buffer.double
            )
        }

        @JvmStatic
        @JvmName("readByteBuffer")
        internal fun sequence(
            buffer: ByteBuffer,
            order: ByteOrder = ByteOrder.LITTLE_ENDIAN
        ): Sequence<TimingPoint> = sequence {
            buffer.order(order)
            while (buffer.hasRemaining()) {
                yield(invoke(buffer))
            }
        }
    }
}