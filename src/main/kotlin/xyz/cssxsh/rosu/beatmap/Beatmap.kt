package xyz.cssxsh.rosu.beatmap

import xyz.cssxsh.rosu.GameMode
import xyz.cssxsh.rosu.Library
import xyz.cssxsh.rosu.pretty
import java.nio.ByteBuffer
import java.nio.ByteOrder

/**
 * [Beatmap](https://osu.ppy.sh/wiki/en/Beatmap) bind
 *
 * @param ptr 指针
 */
public class Beatmap internal constructor(@PublishedApi internal val ptr: Long) : Cloneable {

    /**
     * @param bytes data of file
     */
    public constructor(bytes: ByteArray) : this(ptr = create(bytes))

    /**
     * @param path path of file
     */
    public constructor(path: String) : this(ptr = create(path))

    /**
     * default
     */
    public constructor() : this(ptr = default())

    protected fun finalize(): Unit = destroy(ptr = ptr)

    public override fun clone(): Beatmap = Beatmap(ptr = clone(ptr = ptr))

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    /**
     * [Game Mode](https://osu.ppy.sh/wiki/en/Ranking_Criteria)
     */
    public val mode: GameMode
        get() = GameMode.values()[getMode(ptr = ptr)]

    /**
     * [Version](https://osu.ppy.sh/wiki/zh-tw/osu%21_File_Formats/Osu_%28file_format%29)
     */
    public val version: Int
        get() = getVersion(ptr = ptr)

    /**
     * The amount of circles.
     */
    public val circles: Long
        get() = getNCircles(ptr = ptr)

    /**
     * The amount of sliders.
     */
    public val sliders: Long
        get() = getNSliders(ptr = ptr)

    /**
     * The amount of spinners.
     */
    public val spinners: Long
        get() = getNSpinners(ptr = ptr)

    /**
     * [Approach Rate](https://osu.ppy.sh/wiki/en/Beatmap/Approach_rate)
     */
    public val ar: Float
        get() = getAR(ptr = ptr)

    /**
     * [Overall Difficulty](https://osu.ppy.sh/wiki/en/Beatmap/Difficulty)
     */
    public val od: Float
        get() = getOD(ptr = ptr)

    /**
     * [Circle Size](https://osu.ppy.sh/wiki/en/Beatmap/Circle_size)
     */
    public val cs: Float
        get() = getCS(ptr = ptr)

    /**
     * [HP drain rate](https://osu.ppy.sh/wiki/en/Beatmap/HP_drain_rate)
     */
    public val hp: Float
        get() = getHP(ptr = ptr)

    /**
     * Base slider velocity in pixels per beat
     */
    public val sliderMultiplier: Double
        get() = getSM(ptr = ptr)

    /**
     * Amount of slider ticks per beat.
     */
    public val sliderTickRate: Double
        get() = getTR(ptr = ptr)

    /**
     * Store the sounds for all objects in their own Vec to minimize the struct size. Hit sounds are only used in osu!taiko in which they represent color.
     */
    public val sounds: ByteBuffer
        get() = getSounds(ptr = ptr).order(ByteOrder.LITTLE_ENDIAN)

    /**
     * [Timing](https://osu.ppy.sh/wiki/zh/Beatmapping/Timing) points that indicate a new timing section.
     */
    public val timingPoints: List<TimingPoint>
        get() = TimingPoint.sequence(buffer = getTimingPoints(ptr = ptr).order(ByteOrder.LITTLE_ENDIAN)).toList()

    /**
     * [Timing](https://osu.ppy.sh/wiki/zh/Beatmapping/Timing) point for the current timing section.
     */
    public val difficultyPoints: List<DifficultyPoint>
        get() = DifficultyPoint.sequence(buffer = getDifficultyPoints(ptr = ptr).order(ByteOrder.LITTLE_ENDIAN)).toList()

    /**
     * Control points for effect sections.
     */
    public val effectPoints: List<EffectPoint>
        get() = EffectPoint.sequence(buffer = getEffectPoints(ptr = ptr).order(ByteOrder.LITTLE_ENDIAN)).toList()

    /**
     * The stack leniency that is used to calculate the stack offset for stacked positions.
     */
    public val stackLeniency: Float
        get() = getSL(ptr = ptr)

    /**
     * All break points of the beatmap.
     */
    public val breaks: List<Break>
        get() = Break.sequence(buffer = getBreaks(ptr = ptr).order(ByteOrder.LITTLE_ENDIAN)).toList()

    /**
     * [Beats per minute](https://osu.ppy.sh/wiki/zh/Beatmapping/Beats_per_minute)
     */
    public fun bpm(): Double = bpm(ptr = ptr)

    /**
     * Sum up the duration of all breaks (in milliseconds).
     */
    @JvmName("getTotalBreakTime")
    public fun totalBreakTime(): Double = getTotalBreakTime(ptr = ptr)

    /**
     * Return the TimingPoint for the given timestamp.
     */
    @JvmName("getTimingPointAt")
    public fun timingPointAt(time: Double): TimingPoint {
        return TimingPoint(getTimingPointAt(ptr = ptr, time = time).order(ByteOrder.LITTLE_ENDIAN))
    }

    /**
     * Return the DifficultyPoint for the given timestamp.
     *
     * If time is before the first difficulty point, None is returned.
     */
    @JvmName("getDifficultyPointAt")
    public fun difficultyPointAt(time: Double): DifficultyPoint? {
        val buffer = getDifficultyPointAt(ptr = ptr, time = time) ?: return null
        return DifficultyPoint(buffer = buffer.order(ByteOrder.LITTLE_ENDIAN))
    }

    /**
     * Return the EffectPoint for the given timestamp.
     *
     * If time is before the first effect point, None is returned.
     */
    @JvmName("getEffectPointAt")
    public fun effectPointAt(time: Double): EffectPoint? {
        val buffer = getEffectPointAt(ptr = ptr, time = time) ?: return null
        return EffectPoint(buffer = buffer.order(ByteOrder.LITTLE_ENDIAN))
    }

    /**
     * Convert a Beatmap of some mode into a different mode.
     */
    public fun convertMode(mode: GameMode): Beatmap {
        return Beatmap(ptr = convertMode(ptr = ptr, mode = mode.ordinal))
    }

    public companion object {
        init {
            Library.staticLoad()
        }

        @JvmStatic
        internal external fun create(bytes: ByteArray): Long

        @JvmStatic
        internal external fun create(path: String): Long

        @JvmStatic
        internal external fun default(): Long

        @JvmStatic
        internal external fun clone(ptr: Long): Long

        @JvmStatic
        internal external fun destroy(ptr: Long)

        @JvmStatic
        internal external fun debug(ptr: Long, pretty: Boolean): String

        @JvmStatic
        internal external fun getMode(ptr: Long): Int

        @JvmStatic
        internal external fun getVersion(ptr: Long): Int

        @JvmStatic
        internal external fun getNCircles(ptr: Long): Long

        @JvmStatic
        internal external fun getNSliders(ptr: Long): Long

        @JvmStatic
        internal external fun getNSpinners(ptr: Long): Long

        @JvmStatic
        internal external fun getAR(ptr: Long): Float

        @JvmStatic
        internal external fun getOD(ptr: Long): Float

        @JvmStatic
        internal external fun getCS(ptr: Long): Float

        @JvmStatic
        internal external fun getHP(ptr: Long): Float

        @JvmStatic
        internal external fun getSM(ptr: Long): Double

        @JvmStatic
        internal external fun getTR(ptr: Long): Double

        @JvmStatic
        internal external fun getSL(ptr: Long): Float

        @JvmStatic
        internal external fun getBreaks(ptr: Long): ByteBuffer

        @JvmStatic
        internal external fun bpm(ptr: Long): Double

        @JvmStatic
        internal external fun getTotalBreakTime(ptr: Long): Double

        @JvmStatic
        internal external fun getSounds(ptr: Long): ByteBuffer

        @JvmStatic
        internal external fun getTimingPoints(ptr: Long): ByteBuffer

        @JvmStatic
        internal external fun getTimingPointAt(ptr: Long, time: Double): ByteBuffer

        @JvmStatic
        internal external fun getDifficultyPoints(ptr: Long): ByteBuffer

        @JvmStatic
        internal external fun getDifficultyPointAt(ptr: Long, time: Double): ByteBuffer?

        @JvmStatic
        internal external fun getEffectPoints(ptr: Long): ByteBuffer

        @JvmStatic
        internal external fun getEffectPointAt(ptr: Long, time: Double): ByteBuffer?

        @JvmStatic
        internal external fun convertMode(ptr: Long, mode: Int): Long
    }
}