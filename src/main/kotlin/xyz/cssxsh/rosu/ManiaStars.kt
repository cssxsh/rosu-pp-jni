package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public class ManiaStars @PublishedApi internal constructor(internal val ptr: Long, override val map: Beatmap) :
    AnyStars {
    override val mode: GameMode = GameMode.Mania

    public constructor(map: Beatmap) : this(ptr = create(map = map.ptr), map = map)

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): ManiaStars = ManiaStars(ptr = clone(ptr = ptr), map = map)

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    @ROsuPP
    override fun calculate(): ManiaDifficultyAttributes {
        return ManiaDifficultyAttributes(ptr = calculate(ptr = ptr))
    }

    @ROsuPP
    override fun strains(): ManiaStrains {
        return ManiaStrains(ptr = strains(ptr = ptr))
    }

    @ROsuPP
    override fun mods(flag: Long): ManiaStars = apply {
        withMods(ptr = ptr, flag = flag)
    }

    @ROsuPP
    override fun passedObjects(number: Long): ManiaStars = apply {
        withPasseObjects(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun clockRate(value: Double): ManiaStars = apply {
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
        internal external fun withMods(ptr: Long, flag: Long): Long

        @JvmStatic
        internal external fun withPasseObjects(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withClockRate(ptr: Long, value: Double): Long
    }
}