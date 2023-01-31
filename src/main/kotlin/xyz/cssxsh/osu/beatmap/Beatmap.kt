package xyz.cssxsh.osu.beatmap

import xyz.cssxsh.osu.GameMode
import xyz.cssxsh.osu.Library
import xyz.cssxsh.osu.PRETTY_PRINT_KEY

/**
 * [Beatmap](https://osu.ppy.sh/wiki/en/Beatmap) bind
 *
 * @param ptr 指针
 */
public class Beatmap internal constructor(@PublishedApi internal val ptr: Long): Cloneable {

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

    override fun clone(): Beatmap = Beatmap(ptr = clone(ptr = ptr))

    /**
     * [Game Mode](https://osu.ppy.sh/wiki/en/Ranking_Criteria)
     */
    public var mode: GameMode
        get() = GameMode.values()[getMode(ptr = ptr)]
        set(value) = setMode(ptr = ptr, index = value.ordinal)

    /**
     * [Version](https://osu.ppy.sh/wiki/zh-tw/osu%21_File_Formats/Osu_%28file_format%29)
     */
    public var version: Int
        get() = getVersion(ptr = ptr)
        set(value) = setVersion(ptr = ptr, version = value)

    public var circles: Long
        get() = getNCircles(ptr = ptr)
        set(value) = setNCircles(ptr = ptr, number = value)

    public var sliders: Long
        get() = getNSliders(ptr = ptr)
        set(value) = setNSliders(ptr = ptr, number = value)

    public var spinners: Long
        get() = getNSpinners(ptr = ptr)
        set(value) = setNSpinners(ptr = ptr, number = value)

    /**
     * [Approach Rate](https://osu.ppy.sh/wiki/en/Beatmap/Approach_rate)
     */
    public var ar: Float
        get() = getAR(ptr = ptr)
        set(value) = setAR(ptr = ptr, ar = value)

    /**
     * [Overall Difficulty](https://osu.ppy.sh/wiki/en/Beatmap/Difficulty)
     */
    public var od: Float
        get() = getOD(ptr = ptr)
        set(value) = setOD(ptr = ptr, od = value)

    /**
     * [Circle Size](https://osu.ppy.sh/wiki/en/Beatmap/Circle_size)
     */
    public var cs: Float
        get() = getCS(ptr = ptr)
        set(value) = setCS(ptr = ptr, cs = value)

    /**
     * [HP drain rate](https://osu.ppy.sh/wiki/en/Beatmap/HP_drain_rate)
     */
    public var hp: Float
        get() = getHP(ptr = ptr)
        set(value) = setHP(ptr = ptr, hp = value)

    public var slider_mult: Double = 0.0

    public var tick_rate: Double = 0.0

    public var hit_objects: Double = 0.0

    public var sounds: Double = 0.0

    public var timing_points: Double = 0.0

    public var difficulty_points: Double = 0.0

    public var effect_points: Double = 0.0

    public var stack_leniency: Double = 0.0

    public var breaks: List<Break>
        get() = buildList {
            val array = getBreaks(ptr = ptr)
            for (index in array.indices step 2) {
                Break(startTime = array[index], endTime = array[index + 1])
            }
        }
        set(value) = setBreaks(ptr = ptr, breaks = DoubleArray(value.size * 2).apply {
            var index = 0
            for (item in breaks) {
                set(index++, item.startTime)
                set(index++, item.endTime)
            }
        })

    override fun toString(): String = debug(ptr = ptr, pretty = java.lang.Boolean.getBoolean(PRETTY_PRINT_KEY))

    public companion object {
        init {
            Library.staticLoad()
        }

        internal external fun create(bytes: ByteArray): Long

        internal external fun create(path: String): Long

        internal external fun default(): Long

        internal external fun clone(ptr: Long): Long

        internal external fun destroy(ptr: Long)

        internal external fun debug(ptr: Long, pretty: Boolean): String

        internal external fun getMode(ptr: Long): Int

        internal external fun setMode(ptr: Long, index: Int)

        internal external fun getVersion(ptr: Long): Int

        internal external fun setVersion(ptr: Long, version: Int)

        internal external fun getNCircles(ptr: Long): Long

        internal external fun setNCircles(ptr: Long, number: Long)

        internal external fun getNSliders(ptr: Long): Long

        internal external fun setNSliders(ptr: Long, number: Long)

        internal external fun getNSpinners(ptr: Long): Long

        internal external fun setNSpinners(ptr: Long, number: Long)

        internal external fun getAR(ptr: Long): Float

        internal external fun setAR(ptr: Long, ar: Float)

        internal external fun getOD(ptr: Long): Float

        internal external fun setOD(ptr: Long, od: Float)

        internal external fun getCS(ptr: Long): Float

        internal external fun setCS(ptr: Long, cs: Float)

        internal external fun getHP(ptr: Long): Float

        internal external fun setHP(ptr: Long, hp: Float)

        internal external fun getBreaks(ptr: Long): DoubleArray

        internal external fun setBreaks(ptr: Long, breaks: DoubleArray)
    }
}