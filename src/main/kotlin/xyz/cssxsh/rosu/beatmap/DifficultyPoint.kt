package xyz.cssxsh.rosu.beatmap

import java.nio.ByteBuffer
import java.nio.ByteOrder

public data class DifficultyPoint(
    public val time: Double,
    public val sliderVelocity: Double,
    public val bpmMultiplier: Double,
    public val generateTicks: Boolean,
) {
    public companion object Native {
        @JvmStatic
        @JvmName("fromByteBuffer")
        internal operator fun invoke(buffer: ByteBuffer, order: ByteOrder = ByteOrder.LITTLE_ENDIAN): DifficultyPoint {
            buffer.order(order)
            return DifficultyPoint(
                time = buffer.double,
                sliderVelocity = buffer.double,
                bpmMultiplier = buffer.double,
                generateTicks = (buffer.long and -0x0100_0000_0000_0000L) != 0L,
            )
        }

        @JvmStatic
        @JvmName("readByteBuffer")
        internal fun sequence(
            buffer: ByteBuffer,
            order: ByteOrder = ByteOrder.LITTLE_ENDIAN
        ): Sequence<DifficultyPoint> = sequence {
            buffer.order(order)
            while (buffer.hasRemaining()) {
                yield(invoke(buffer))
            }
        }
    }
}