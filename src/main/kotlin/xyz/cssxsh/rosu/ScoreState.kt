package xyz.cssxsh.rosu

public class ScoreState @PublishedApi internal constructor(internal val ptr: Long) : Cloneable {

    public constructor(): this(ptr = default())

    protected fun finalize(): Unit = destroy(ptr = ptr)

    override fun clone(): ScoreState = ScoreState(ptr = clone(ptr = ptr))

    override fun toString(): String = debug(ptr = ptr, pretty = pretty())

    @ROsuPP
    public fun totalHits(mode: GameMode): Long = totalHits(ptr = ptr, mode = mode.ordinal)

    @ROsuPP
    public fun accuracy(mode: GameMode): Double = accuracy(ptr = ptr, mode = mode.ordinal)

    @ROsuPP
    public var maxCombo: Long
        get() = getMaxCombo(ptr = ptr)
        set(value) = setMaxCombo(ptr = ptr, number = value)

    @ROsuPP
    public fun maxCombo(number: Long): ScoreState = apply { maxCombo = number }

    @ROsuPP
    public var geki: Long
        get() = getNGeki(ptr = ptr)
        set(value) = setNGeki(ptr = ptr, number = value)

    @ROsuPP
    public fun geki(number: Long): ScoreState = apply { geki = number }

    @ROsuPP
    public var katu: Long
        get() = getNKatu(ptr = ptr)
        set(value) = setNKatu(ptr = ptr, number = value)

    @ROsuPP
    public fun katu(number: Long): ScoreState = apply { katu = number }

    @ROsuPP
    public var n300: Long
        get() = getN300(ptr = ptr)
        set(value) = setN300(ptr = ptr, number = value)

    @ROsuPP
    public fun n300(number: Long): ScoreState = apply { n300 = number }

    @ROsuPP
    public var n100: Long
        get() = getN100(ptr = ptr)
        set(value) = setN100(ptr = ptr, number = value)

    @ROsuPP
    public fun n100(number: Long): ScoreState = apply { n100 = number }

    @ROsuPP
    public var n50: Long
        get() = getN50(ptr = ptr)
        set(value) = setN50(ptr = ptr, number = value)

    @ROsuPP
    public fun n50(number: Long): ScoreState = apply { n50 = number }

    @ROsuPP
    public var misses: Long
        get() = getNMisses(ptr = ptr)
        set(value) = setNMisses(ptr = ptr, number = value)

    @ROsuPP
    public fun misses(number: Long): ScoreState = apply { misses = number }

    public companion object Native {
        init {
            Library.staticLoad()
        }

        @JvmStatic
        internal external fun default(): NativePointer

        @JvmStatic
        internal external fun clone(ptr: NativePointer): NativePointer

        @JvmStatic
        internal external fun destroy(ptr: NativePointer)

        @JvmStatic
        internal external fun debug(ptr: NativePointer, pretty: Boolean): String

        @JvmStatic
        internal external fun totalHits(ptr: NativePointer, mode: Int): Long

        @JvmStatic
        internal external fun accuracy(ptr: NativePointer, mode: Int): Double

        @JvmStatic
        internal external fun getMaxCombo(ptr: NativePointer): Long

        @JvmStatic
        internal external fun setMaxCombo(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun getNGeki(ptr: NativePointer): Long

        @JvmStatic
        internal external fun setNGeki(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun getNKatu(ptr: NativePointer): Long

        @JvmStatic
        internal external fun setNKatu(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun getN300(ptr: NativePointer): Long

        @JvmStatic
        internal external fun setN300(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun getN100(ptr: NativePointer): Long

        @JvmStatic
        internal external fun setN100(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun getN50(ptr: NativePointer): Long

        @JvmStatic
        internal external fun setN50(ptr: NativePointer, number: Long)

        @JvmStatic
        internal external fun getNMisses(ptr: NativePointer): Long

        @JvmStatic
        internal external fun setNMisses(ptr: NativePointer, number: Long)
    }
}