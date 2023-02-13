package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public class OsuPP @PublishedApi internal constructor(internal val ptr: Long, override val map: Beatmap) :
    AnyPP {
    override val mode: GameMode = GameMode.Osu

    internal constructor(map: Beatmap) : this(ptr = create(map = map.ptr), map = map)

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): OsuPP = OsuPP(ptr = clone(ptr = ptr), map = map)

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    @ROsuPP
    override fun calculate(): OsuPerformanceAttributes {
        return OsuPerformanceAttributes(ptr = calculate(ptr = ptr))
    }

    @ROsuPP
    override fun attributes(value: PerformanceAttributes): OsuPP = apply {
        withAttributes(ptr = ptr, value = value.ptr, mode = value.mode.ordinal)
    }

    @ROsuPP
    public fun hitResultPriority(priority: HitResultPriority): OsuPP = apply {
        withHitResultPriority(ptr = ptr, index = priority.ordinal)
    }

    @ROsuPP
    override fun mode(target: GameMode): AnyPP {
        val ptr = convertMode(ptr = ptr, mode = target.ordinal)
        return when (target) {
            GameMode.Osu -> OsuPP(ptr = ptr, map = map)
            GameMode.Taiko -> TaikoPP(ptr = ptr, map = map)
            GameMode.Catch -> CatchPP(ptr = ptr, map = map)
            GameMode.Mania -> ManiaPP(ptr = ptr, map = map)
        }
    }

    @ROsuPP
    override fun mods(flag: Long): OsuPP = apply {
        withMods(ptr = ptr, flag = flag)
    }

    @ROsuPP
    override fun passedObjects(number: Long): OsuPP = apply {
        withPasseObjects(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun clockRate(rate: Double): OsuPP = apply {
        withClockRate(ptr = ptr, value = rate)
    }

    @ROsuPP
    override fun accuracy(value: Double): OsuPP = apply {
        withAccuracy(ptr = ptr, value = value)
    }

    @ROsuPP
    override fun misses(number: Long): OsuPP = apply {
        withNMisses(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun combo(number: Long): OsuPP = apply {
        withCombo(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun n300(number: Long): OsuPP = apply {
        withN300(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun n100(number: Long): OsuPP = apply {
        withN100(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun n50(number: Long): OsuPP = apply {
        withN50(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun katu(number: Long): OsuPP = this

    @ROsuPP
    override fun geki(number: Long): OsuPP = this

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
        internal external fun convertMode(ptr: Long, mode: Int): Long

        @JvmStatic
        internal external fun withAttributes(ptr: Long, value: Long, mode: Int)

        @JvmStatic
        internal external fun withMods(ptr: Long, flag: Long)

        @JvmStatic
        internal external fun withCombo(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withHitResultPriority(ptr: Long, index: Int): Long

        @JvmStatic
        internal external fun withN300(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withN100(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withN50(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withNMisses(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withPasseObjects(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withClockRate(ptr: Long, value: Double): Long

        @JvmStatic
        internal external fun withAccuracy(ptr: Long, value: Double): Long
    }
}