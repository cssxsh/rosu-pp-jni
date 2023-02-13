package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public class ManiaPP internal constructor(@PublishedApi internal val ptr: Long, override val map: Beatmap) : AnyPP {
    override val mode: GameMode = GameMode.Mania

    internal constructor(map: Beatmap) : this(ptr = create(map = map.ptr), map = map)

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): CatchPP = CatchPP(ptr = clone(ptr = ptr), map = map)

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    @ROsuPP
    override fun calculate(): MainaPerformanceAttributes {
        return MainaPerformanceAttributes(ptr = calculate(ptr = ptr))
    }

    @ROsuPP
    override fun mods(value: Long): ManiaPP = apply {
        withMods(ptr = ptr, value = value)
    }

    @ROsuPP
    override fun passedObjects(number: Long): ManiaPP = apply {
        withPasseObjects(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun clockRate(rate: Double): ManiaPP = apply {
        withClockRate(ptr = ptr, value = rate)
    }

    @ROsuPP
    override fun accuracy(value: Double): ManiaPP = apply {
        withAccuracy(ptr = ptr, value = value)
    }

    @ROsuPP
    override fun misses(number: Long): ManiaPP = apply {
        withNMisses(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun combo(number: Long): ManiaPP = this

    @ROsuPP
    public fun convert(value: Boolean): ManiaPP = apply {
        withIsConvert(ptr = ptr, value = value)
    }

    @ROsuPP
    override fun n300(number: Long): ManiaPP = apply {
        withN300(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun n100(number: Long): ManiaPP = apply {
        withN100(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun n50(number: Long): ManiaPP = apply {
        withN50(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun katu(number: Long): ManiaPP = apply {
        withN200(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun geki(number: Long): ManiaPP= apply {
        withN320(ptr = ptr, number = number)
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
        internal external fun withMods(ptr: Long, value: Long)

        @JvmStatic
        internal external fun withHitResultPriority(ptr: Long, index: Int): Long

        @JvmStatic
        internal external fun withN300(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withN100(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withN50(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withN200(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withN320(ptr: Long, number: Long): Long

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