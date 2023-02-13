package xyz.cssxsh.rosu.beatmap

import java.nio.ByteBuffer
import java.nio.ByteOrder

public data class EffectPoint(
    public val time: Double,
    public val kiai: Boolean
) {
    public companion object Native {
        @JvmStatic
        @JvmName("fromByteBuffer")
        internal operator fun invoke(buffer: ByteBuffer, order: ByteOrder = ByteOrder.LITTLE_ENDIAN): EffectPoint {
            buffer.order(order)
            return EffectPoint(
                time = buffer.double,
                kiai = (buffer.long and -0x0100_0000_0000_0000L) != 0L
            )
        }

        @JvmStatic
        @JvmName("readByteBuffer")
        internal fun sequence(
            buffer: ByteBuffer,
            order: ByteOrder = ByteOrder.LITTLE_ENDIAN
        ): Sequence<EffectPoint> = sequence {
            buffer.order(order)
            while (buffer.hasRemaining()) {
                yield(invoke(buffer))
            }
        }
    }
}