package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public class TaikoPP internal constructor(@PublishedApi internal val ptr: Long, override val map: Beatmap) : AnyPP {
    override val mode: GameMode = GameMode.Taiko

    internal constructor(map: Beatmap) : this(ptr = create(map = map.ptr), map = map)

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): TaikoPP = TaikoPP(ptr = clone(ptr = ptr), map = map)

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    @ROsuPP
    override fun calculate(): TaikoPerformanceAttributes {
        return TaikoPerformanceAttributes(ptr = calculate(ptr = ptr))
    }

    @ROsuPP
    public fun hitResultPriority(priority: HitResultPriority): TaikoPP = apply {
        withHitResultPriority(ptr = ptr, index = priority.ordinal)
    }

    @ROsuPP
    public fun convert(value: Boolean): TaikoPP = apply {
        withIsConvert(ptr = ptr, value = value)
    }

    @ROsuPP
    override fun mods(value: Long): TaikoPP = apply {
        withMods(ptr = ptr, value = value)
    }

    @ROsuPP
    override fun passedObjects(number: Long): TaikoPP = apply {
        withPasseObjects(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun clockRate(rate: Double): TaikoPP = apply {
        withClockRate(ptr = ptr, value = rate)
    }

    @ROsuPP
    override fun accuracy(value: Double): TaikoPP = apply {
        withAccuracy(ptr = ptr, value = value)
    }

    @ROsuPP
    override fun misses(number: Long): TaikoPP = apply {
        withNMisses(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun combo(number: Long): TaikoPP = apply {
        withCombo(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun n300(number: Long): TaikoPP = apply {
        withN300(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun n100(number: Long): TaikoPP = apply {
        withN100(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun n50(number: Long): AnyPP = this

    @ROsuPP
    override fun katu(number: Long): AnyPP = this

    @ROsuPP
    override fun geki(number: Long): AnyPP = this

    public companion object {
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
        internal external fun withMods(ptr: Long, value: Long)

        @JvmStatic
        internal external fun withCombo(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withHitResultPriority(ptr: Long, index: Int): Long

        @JvmStatic
        internal external fun withN300(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withN100(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withNMisses(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withPasseObjects(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withClockRate(ptr: Long, value: Double): Long

        @JvmStatic
        internal external fun withAccuracy(ptr: Long, value: Double): Long

        @JvmStatic
        internal external fun withIsConvert(ptr: Long, value: Boolean): Long
    }
}