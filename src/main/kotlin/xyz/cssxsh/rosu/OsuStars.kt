package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public class OsuStars @PublishedApi internal constructor(internal val ptr: Long, override val map: Beatmap) :
    AnyStars {
    override val mode: GameMode = GameMode.Osu

    public constructor(map: Beatmap) : this(ptr = create(map = map.ptr), map = map)

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): OsuStars = OsuStars(ptr = clone(ptr = ptr), map = map)

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    @ROsuPP
    override fun calculate(): OsuDifficultyAttributes {
        return OsuDifficultyAttributes(ptr = calculate(ptr = ptr))
    }

    @ROsuPP
    override fun strains(): OsuStrains {
        return OsuStrains(ptr = strains(ptr = ptr))
    }

    @ROsuPP
    override fun mode(target: GameMode): AnyStars {
        val ptr = convertMode(ptr = ptr, mode = target.ordinal)
        return when (target) {
            GameMode.Osu -> OsuStars(ptr = ptr, map = map)
            GameMode.Taiko -> TaikoStars(ptr = ptr, map = map)
            GameMode.Catch -> CatchStars(ptr = ptr, map = map)
            GameMode.Mania -> ManiaStars(ptr = ptr, map = map)
        }
    }

    @ROsuPP
    override fun mods(flag: Long): OsuStars = apply {
        withMods(ptr = ptr, flag = flag)
    }

    @ROsuPP
    override fun passedObjects(number: Long): OsuStars = apply {
        withPasseObjects(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun clockRate(value: Double): OsuStars = apply {
        withClockRate(ptr = ptr, value = value)
    }

    public companion object Native {
        init {
            Library.staticLoad()
        }

        @JvmStatic
        internal external fun create(map: Long): Long

        @JvmStatic
        internal external fun clone(ptr: Long): Long

        @JvmStatic
        internal external fun destroy(ptr: Long)

        @JvmStatic
        internal external fun debug(ptr: Long, pretty: Boolean): String

        @JvmStatic
        internal external fun calculate(ptr: Long): Long

        @JvmStatic
        internal external fun strains(ptr: Long): Long

        @JvmStatic
        internal external fun convertMode(ptr: Long, mode: Int): Long

        @JvmStatic
        internal external fun withMods(ptr: Long, flag: Long): Long

        @JvmStatic
        internal external fun withPasseObjects(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withClockRate(ptr: Long, value: Double): Long
    }
}