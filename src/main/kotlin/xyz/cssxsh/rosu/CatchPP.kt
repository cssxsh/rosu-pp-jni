package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public class CatchPP @PublishedApi internal constructor(internal val ptr: NativePointer, override val map: Beatmap) :
    AnyPP {
    override val mode: GameMode = GameMode.Catch

    internal constructor(map: Beatmap) : this(ptr = create(map = map.ptr), map = map)

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): CatchPP = CatchPP(ptr = clone(ptr = ptr), map = map)

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    @ROsuPP
    override fun calculate(): CatchPerformanceAttributes {
        return CatchPerformanceAttributes(ptr = calculate(ptr = ptr))
    }

    @ROsuPP
    override fun attributes(value: PerformanceAttributes): CatchPP = apply {
        withPerformanceAttributes(ptr = ptr, value = value.ptr, mode = value.mode.ordinal)
    }

    @ROsuPP
    override fun attributes(value: DifficultyAttributes): CatchPP = apply {
        withDifficultyAttributes(ptr = ptr, value = value.ptr, mode = value.mode.ordinal)
    }

    @ROsuPP
    override fun mods(flag: Long): CatchPP = apply {
        withMods(ptr = ptr, flag = flag)
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
        internal external fun withPerformanceAttributes(ptr: NativePointer, value: NativePointer, mode: Int)

        @JvmStatic
        internal external fun withMods(ptr: NativePointer, flag: Long)

        @JvmStatic
        internal external fun withCombo(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withFruits(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withDroplets(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withTinyDropletMisses(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withTinyDroplets(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withMisses(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withPasseObjects(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withClockRate(ptr: NativePointer, value: Double)

        @JvmStatic
        internal external fun withAccuracy(ptr: Long, value: Double): Long

        @JvmStatic
        internal external fun withDifficultyAttributes(ptr: NativePointer, value: NativePointer, mode: Int)
    }
}