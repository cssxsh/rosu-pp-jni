package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public sealed interface AnyPP : Cloneable {

    public val mode: GameMode

    public val map: Beatmap

    @ROsuPP
    public fun calculate(): PerformanceAttributes

    @ROsuPP
    public fun attributes(value: PerformanceAttributes): AnyPP

    @ROsuPP
    public fun attributes(value: DifficultyAttributes): AnyPP

    @ROsuPP
    public fun mode(target: GameMode): AnyPP = this

    @ROsuPP
    public fun mods(flag: Long): AnyPP

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

    @ROsuPP
    public fun convert(value: Boolean): AnyPP = this

    public companion object Native {
        init {
            Library.staticLoad()
        }

        @JvmStatic
        @JvmName("create")
        public operator fun invoke(map: Beatmap): AnyPP {
            return when (map.mode) {
                GameMode.Osu -> OsuPP(map = map)
                GameMode.Taiko -> TaikoPP(map = map)
                GameMode.Catch -> CatchPP(map = map)
                GameMode.Mania -> ManiaPP(map = map)
            }
        }

        @JvmStatic
        @JvmName("map")
        public fun map(bytes: ByteArray): AnyPP {
            val map = Beatmap(bytes = bytes)
            return when (map.mode) {
                GameMode.Osu -> OsuPP(map = map)
                GameMode.Taiko -> TaikoPP(map = map)
                GameMode.Catch -> CatchPP(map = map)
                GameMode.Mania -> ManiaPP(map = map)
            }
        }

        @JvmStatic
        @JvmName("map")
        public fun map(path: String): AnyPP {
            val map = Beatmap(path = path)
            return when (map.mode) {
                GameMode.Osu -> OsuPP(map = map)
                GameMode.Taiko -> TaikoPP(map = map)
                GameMode.Catch -> CatchPP(map = map)
                GameMode.Mania -> ManiaPP(map = map)
            }
        }
    }
}
