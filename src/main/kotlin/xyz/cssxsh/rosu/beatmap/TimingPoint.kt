package xyz.cssxsh.rosu.beatmap

import java.nio.ByteBuffer

public data class TimingPoint(
    val beatLen: Double,
    val time: Double
) {
    public companion object {
        @JvmStatic
        @JvmName("fromByteBuffer")
        internal operator fun invoke(buffer: ByteBuffer): TimingPoint {
            return TimingPoint(
                beatLen = buffer.double,
                time = buffer.double
            )
        }

        @JvmStatic
        @JvmName("readByteBuffer")
        internal fun sequence(buffer: ByteBuffer): Sequence<TimingPoint> = sequence {
            while (buffer.hasRemaining()) {
                yield(invoke(buffer))
            }
        }
    }
}