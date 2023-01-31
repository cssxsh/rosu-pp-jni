package xyz.cssxsh.osu.rust

/**
 * [Beatmap](https://osu.ppy.sh/wiki/en/Beatmap) bind
 *
 * @param ptr 指针
 */
public class Beatmap internal constructor(@PublishedApi internal val ptr: Long) {

    /**
     * @param bytes data of file
     */
    public constructor(bytes: ByteArray): this(ptr = create(bytes))

    /**
     * @param path path of file
     */
    public constructor(path: String): this(ptr = create(path))

    protected fun finalize(): Unit = destroy(ptr = ptr)

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

    public var breaks: List<Break>
        get() = TODO()
        set(value) = TODO()

    override fun toString(): String {
        return "Beatmap(version=${version}, mode=${mode}, ar=${ar}, od=${od}, cs=${cs}, hp=${hp})"
    }

    public companion object {
        init {
            Library.staticLoad()
        }

        internal external fun create(bytes: ByteArray): Long

        internal external fun create(path: String): Long

        internal external fun destroy(ptr: Long)

        internal external fun getMode(ptr: Long): Int

        internal external fun setMode(ptr: Long, index: Int)

        internal external fun getVersion(ptr: Long): Int

        internal external fun setVersion(ptr: Long, version: Int)

        internal external fun getAR(ptr: Long): Float

        internal external fun setAR(ptr: Long, ar: Float)

        internal external fun getOD(ptr: Long): Float

        internal external fun setOD(ptr: Long, od: Float)

        internal external fun getCS(ptr: Long): Float

        internal external fun setCS(ptr: Long, cs: Float)

        internal external fun getHP(ptr: Long): Float

        internal external fun setHP(ptr: Long, hp: Float)
    }
}