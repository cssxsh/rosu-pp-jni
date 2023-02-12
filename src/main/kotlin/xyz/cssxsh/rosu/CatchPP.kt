package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public class CatchPP internal constructor(@PublishedApi internal var ptr: Long, override val map: Beatmap) : AnyPP {
    override val mode: GameMode = GameMode.Osu

    internal constructor(map: Beatmap) : this(ptr = create(map = map.ptr), map = map)

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): CatchPP = CatchPP(ptr = clone(ptr = ptr), map = map)

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    @ROsuPP
    override fun calculate(): CatchPerformanceAttributes {
        return CatchPerformanceAttributes(ptr = calculate(ptr = ptr))
    }

    @ROsuPP
    override fun mods(value: Long): CatchPP = apply {
        withMods(ptr = ptr, value = value)
    }

    @ROsuPP
    override fun passedObjects(number: Long): CatchPP = apply {
        withPasseObjects(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun clockRate(rate: Double): CatchPP = apply {
        withClockRate(ptr = ptr, value = rate)
    }

    @ROsuPP
    override fun accuracy(value: Double): CatchPP = apply {
        withAccuracy(ptr = ptr, value = value)
    }

    @ROsuPP
    override fun misses(number: Long): CatchPP = apply {
        withMisses(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun combo(number: Long): CatchPP = apply {
        withCombo(ptr = ptr, number = number)
    }

    @ROsuPP
    override fun n300(number: Long): CatchPP = apply {
        withFruits(ptr = ptr, number = number)
    }

    @ROsuPP
    public fun fruits(number: Long): CatchPP = n300(number = number)

    @ROsuPP
    override fun n100(number: Long): CatchPP = apply {
        withDroplets(ptr = ptr, number = number)
    }

    @ROsuPP
    public fun droplets(number: Long): CatchPP = n100(number = number)

    @ROsuPP
    override fun n50(number: Long): CatchPP = apply {
        withTinyDroplets(ptr = ptr, number = number)
    }

    @ROsuPP
    public fun tinyDroplets(number: Long): CatchPP = n100(number = number)

    @ROsuPP
    override fun katu(number: Long): CatchPP = apply {
        withTinyDropletMisses(ptr = ptr, number = number)
    }

    @ROsuPP
    public fun tinyDropletMisses(number: Long): CatchPP = katu(number = number)

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
        internal external fun withFruits(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withDroplets(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withTinyDropletMisses(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withTinyDroplets(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withMisses(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withPasseObjects(ptr: Long, number: Long): Long

        @JvmStatic
        internal external fun withClockRate(ptr: Long, value: Double): Long

        @JvmStatic
        internal external fun withAccuracy(ptr: Long, value: Double): Long
    }
}