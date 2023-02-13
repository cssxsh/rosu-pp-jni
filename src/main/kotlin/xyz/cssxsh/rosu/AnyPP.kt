package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public sealed interface AnyPP : Cloneable {

    public val mode: GameMode

    public val map: Beatmap get() = Beatmap()

    @ROsuPP
    public fun calculate(): PerformanceAttributes

    @ROsuPP
    public fun mode(target: GameMode): AnyPP = this

    @ROsuPP
    public fun mods(value: Long): AnyPP

    @ROsuPP
    public fun passedObjects(number: Long): AnyPP

    @ROsuPP
    public fun clockRate(rate: Double): AnyPP

    @ROsuPP
    public fun accuracy(value: Double): AnyPP

    @ROsuPP
    public fun misses(number: Long): AnyPP

    @ROsuPP
    public fun combo(number: Long): AnyPP

    @ROsuPP
    public fun n300(number: Long): AnyPP

    @ROsuPP
    public fun n100(number: Long): AnyPP

    @ROsuPP
    public fun n50(number: Long): AnyPP

    @ROsuPP
    public fun katu(number: Long): AnyPP

    @ROsuPP
    public fun geki(number: Long): AnyPP

    public companion object Native {
        init {
            Library.staticLoad()
        }

        @JvmStatic
        @JvmName("create")
        public operator fun invoke(map: Beatmap): AnyPP {
            return when (map.mode) {
                GameMode.Osu -> OsuPP(map)
                GameMode.Taiko -> TaikoPP(map)
                GameMode.Catch -> CatchPP(map)
                GameMode.Mania -> ManiaPP(map)
            }
        }

        @JvmStatic
        @JvmName("map")
        public fun map(bytes: ByteArray): AnyPP {
            val map = Beatmap(bytes)
            return when (map.mode) {
                GameMode.Osu -> OsuPP(map)
                GameMode.Taiko -> TaikoPP(map)
                GameMode.Catch -> CatchPP(map)
                GameMode.Mania -> ManiaPP(map)
            }
        }

        @JvmStatic
        @JvmName("map")
        public fun map(path: String): AnyPP {
            val map = Beatmap(path)
            return when (map.mode) {
                GameMode.Osu -> OsuPP(map)
                GameMode.Taiko -> TaikoPP(map)
                GameMode.Catch -> CatchPP(map)
                GameMode.Mania -> ManiaPP(map)
            }
        }
    }
}
