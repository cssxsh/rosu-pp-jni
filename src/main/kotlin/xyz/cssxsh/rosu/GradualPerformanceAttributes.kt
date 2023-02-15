package xyz.cssxsh.rosu

import xyz.cssxsh.rosu.beatmap.Beatmap

public sealed class GradualPerformanceAttributes(public val mode: GameMode) {
    internal abstract val ptr: Long
    public abstract val map: Beatmap

    public fun asSequence(state: ScoreState) : Sequence<PerformanceAttributes> {
        return sequence {
            while (true) {
                yield(processNext(state) ?: break)
            }
        }
    }

    @ROsuPP
    public abstract fun processNext(state: ScoreState, number: Int): PerformanceAttributes?

    @ROsuPP
    public fun processNext(state: ScoreState): PerformanceAttributes? {
        return processNext(state = state, number = 1)
    }

    public companion object Native {
        @JvmStatic
        @JvmName("create")
        public operator fun invoke(map: Beatmap, mods: Long): GradualPerformanceAttributes {
            return when (map.mode) {
                GameMode.Osu -> OsuGradualPerformanceAttributes(map = map, mods = mods)
                GameMode.Taiko -> TaikoGradualPerformanceAttributes(map = map, mods = mods)
                GameMode.Catch -> CatchGradualPerformanceAttributes(map = map, mods = mods)
                GameMode.Mania -> ManiaGradualPerformanceAttributes(map = map, mods = mods)
            }
        }
    }
}
