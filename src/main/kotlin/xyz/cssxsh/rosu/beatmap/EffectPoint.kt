package xyz.cssxsh.rosu.beatmap

import java.nio.ByteBuffer

public data class EffectPoint(
    public val time: Double,
    public val kiai: Boolean
) {
    public companion object {
        @JvmStatic
        @JvmName("fromByteBuffer")
        internal operator fun invoke(buffer: ByteBuffer): EffectPoint {
            return EffectPoint(
                time = buffer.double,
                kiai = (buffer.long and -0x0100_0000_0000_0000L) != 0L
            )
        }

        @JvmStatic
        @JvmName("readByteBuffer")
        internal fun sequence(buffer: ByteBuffer): Sequence<EffectPoint> = sequence {
            while (buffer.hasRemaining()) {
                yield(invoke(buffer))
            }
        }
    }
}