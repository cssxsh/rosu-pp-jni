package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public class TaikoPP @PublishedApi internal constructor(internal val ptr: NativePointer, override val map: Beatmap) :
    AnyPP {
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
    override fun attributes(value: PerformanceAttributes): TaikoPP = apply {
        withPerformanceAttributes(ptr = ptr, value = value.ptr, mode = value.mode.ordinal)
    }

    @ROsuPP
    override fun attributes(value: DifficultyAttributes): TaikoPP = apply {
        withDifficultyAttributes(ptr = ptr, value = value.ptr, mode = value.mode.ordinal)
    }

    @ROsuPP
    override fun mods(flag: Long): TaikoPP = apply {
        withMods(ptr = ptr, flag = flag)
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
        internal external fun withHitResultPriority(ptr: NativePointer, index: Int)

        @JvmStatic
        internal external fun withN300(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun withN100(ptr: NativePointer, number: Long)

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