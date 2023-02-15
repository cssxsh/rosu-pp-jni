package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public class OsuPP @PublishedApi internal constructor(internal val ptr: NativePointer, override val map: Beatmap) :
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
        withPerformanceAttributes(ptr = ptr, value = value.ptr, mode = value.mode.ordinal)
    }

    @ROsuPP
    override fun attributes(value: DifficultyAttributes): OsuPP = apply {
        withDifficultyAttributes(ptr = ptr, value = value.ptr, mode = value.mode.ordinal)
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
    override fun state(state: ScoreState): OsuPP = apply {
        withScoreState(ptr = ptr, state = state.ptr)
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
        internal external fun create(map: NativePointer): NativePointer

        @JvmStatic
        internal external fun clone(ptr: NativePointer): NativePointer

        @JvmStatic
        internal external fun destroy(ptr: NativePointer)

        @JvmStatic
        internal external fun debug(ptr: NativePointer, pretty: Boolean): String

        @JvmStatic
        internal external fun calculate(ptr: NativePointer): NativePointer

        @JvmStatic
        internal external fun convertMode(ptr: NativePointer, mode: Int): NativePointer

        @JvmStatic
        internal external fun withPerformanceAttributes(ptr: NativePointer, value: NativePointer, mode: Int)

        @JvmStatic
        internal external fun withMods(ptr: NativePointer, flag: Long)

        @JvmStatic
        internal external fun withCombo(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withHitResultPriority(ptr: NativePointer, index: Int)

        @JvmStatic
        internal external fun withN300(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withN100(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withN50(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withNMisses(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withPasseObjects(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withClockRate(ptr: NativePointer, value: Double)

        @JvmStatic
        internal external fun withAccuracy(ptr: NativePointer, value: Double)

        @JvmStatic
        internal external fun withDifficultyAttributes(ptr: NativePointer, value: NativePointer, mode: Int)

        @JvmStatic
        internal external fun withScoreState(ptr: NativePointer, state: NativePointer)
    }
}