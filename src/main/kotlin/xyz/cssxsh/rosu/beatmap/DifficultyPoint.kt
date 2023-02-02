package xyz.cssxsh.rosu.beatmap

import java.nio.ByteBuffer

public data class DifficultyPoint(
    public val time: Double,
    public val sliderVelocity: Double,
    public val bpmMultiplier: Double,
    public val generateTicks: Boolean,
) {
    public companion object {
        @JvmStatic
        @JvmName("fromByteBuffer")
        internal operator fun invoke(buffer: ByteBuffer): DifficultyPoint {
            return DifficultyPoint(
                time = buffer.double,
                sliderVelocity = buffer.double,
                bpmMultiplier = buffer.double,
                generateTicks = (buffer.long and -0x0100_0000_0000_0000L) != 0L,
            )
        }

        @JvmStatic
        @JvmName("readByteBuffer")
        internal fun sequence(buffer: ByteBuffer): Sequence<DifficultyPoint> = sequence {
            while (buffer.hasRemaining()) {
                yield(invoke(buffer))
            }
        }
    }
}