package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public class ManiaPP @PublishedApi internal constructor(internal val ptr: NativePointer, override val map: Beatmap) :
    AnyPP {
    override val mode: GameMode = GameMode.Mania

    internal constructor(map: Beatmap) : this(ptr = create(map = map.ptr), map = map)

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): CatchPP = CatchPP(ptr = clone(ptr = ptr), map = map)

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    @ROsuPP
    override fun calculate(): ManiaPerformanceAttributes {
        return ManiaPerformanceAttributes(ptr = calculate(ptr = ptr))
    }

    @ROsuPP
    override fun attributes(value: PerformanceAttributes): ManiaPP = apply {
        withPerformanceAttributes(ptr = ptr, value = value.ptr, mode = value.mode.ordinal)
    }

    @ROsuPP
    override fun attributes(value: DifficultyAttributes): ManiaPP = apply {
        withDifficultyAttributes(ptr = ptr, value = value.ptr, mode = value.mode.ordinal)
    }

    @ROsuPP
    public fun hitResultPriority(priority: HitResultPriority): ManiaPP = apply {
        withHitResultPriority(ptr = ptr, index = priority.ordinal)
    }

    @ROsuPP
    override fun mods(flag: Long): ManiaPP = apply {
        withMods(ptr = ptr, flag = flag)
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
    override fun convert(value: Boolean): ManiaPP = apply {
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
    override fun geki(number: Long): ManiaPP = apply {
        withN320(ptr = ptr, number = number)
    }

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
        internal external fun withHitResultPriority(ptr: NativePointer, index: Int)

        @JvmStatic
        internal external fun withN300(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withN100(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withN50(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withN200(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withN320(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withNMisses(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withPasseObjects(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withClockRate(ptr: NativePointer, value: Double)

        @JvmStatic
        internal external fun withAccuracy(ptr: NativePointer, value: Double)

        @JvmStatic
        internal external fun withIsConvert(ptr: NativePointer, value: Boolean)

        @JvmStatic
        internal external fun withDifficultyAttributes(ptr: NativePointer, value: NativePointer, mode: Int)
    }
}