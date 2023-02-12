package xyz.cssxsh.rosu.beatmap

import xyz.cssxsh.rosu.GameMode
import java.io.File
import kotlin.test.*

internal class BeatmapTest {

    private fun assertBeatmap(expected: GameMode, map: Beatmap) {
        assertNotEquals(0, map.ptr)
        assertTrue(expected.name in map.toString())
        when (expected) {
            GameMode.Osu -> {
                assertEquals(GameMode.Osu, map.mode)
                assertEquals(14, map.version)
                assertEquals(307, map.circles)
                assertEquals(293, map.sliders)
                assertEquals(1, map.spinners)
                assertEquals(9.3F, map.ar)
                assertEquals(8.8F, map.od)
                assertEquals(4.5F, map.cs)
                assertEquals(5.0F, map.hp)
                assertEquals(1.7, map.sliderMultiplier)
                assertEquals(1.0, map.sliderTickRate)
                assertEquals(601, map.sounds.limit())
                assertEquals(1, map.timingPoints.size)
                assertEquals(50, map.difficultyPoints.size)
                assertEquals(131, map.effectPoints.size)
                assertEquals(0.5F, map.stackLeniency)
                assertEquals(1, map.breaks.size)
            }
            GameMode.Taiko -> {
                assertEquals(GameMode.Taiko, map.mode)
                assertEquals(14, map.version)
                assertEquals(289, map.circles)
                assertEquals(4, map.sliders)
                assertEquals(2, map.spinners)
                assertEquals(8.0F, map.ar)
                assertEquals(5.0F, map.od)
                assertEquals(2.0F, map.cs)
                assertEquals(6.0F, map.hp)
                assertEquals(1.4, map.sliderMultiplier)
                assertEquals(1.0, map.sliderTickRate)
                assertEquals(295, map.sounds.limit())
                assertEquals(1, map.timingPoints.size)
                assertEquals(3, map.difficultyPoints.size)
                assertEquals(8, map.effectPoints.size)
                assertEquals(0.7F, map.stackLeniency)
                assertEquals(0, map.breaks.size)
            }
            GameMode.Catch -> {
                assertEquals(GameMode.Catch, map.mode)
                assertEquals(14, map.version)
                assertEquals(249, map.circles)
                assertEquals(227, map.sliders)
                assertEquals(1, map.spinners)
                assertEquals(8.0F, map.ar)
                assertEquals(8.0F, map.od)
                assertEquals(3.5F, map.cs)
                assertEquals(5.0F, map.hp)
                assertEquals(1.45, map.sliderMultiplier)
                assertEquals(1.0, map.sliderTickRate)
                assertEquals(477, map.sounds.limit())
                assertEquals(1, map.timingPoints.size)
                assertEquals(0, map.difficultyPoints.size)
                assertEquals(57, map.effectPoints.size)
                assertEquals(0.7F, map.stackLeniency)
                assertEquals(0, map.breaks.size)
            }
            GameMode.Mania -> {
                assertEquals(GameMode.Mania, map.mode)
                assertEquals(14, map.version)
                assertEquals(2815, map.circles)
                assertEquals(423, map.sliders)
                assertEquals(0, map.spinners)
                assertEquals(5.0F, map.ar)
                assertEquals(8.0F, map.od)
                assertEquals(4.0F, map.cs)
                assertEquals(9.0F, map.hp)
                assertEquals(1.4, map.sliderMultiplier)
                assertEquals(1.0, map.sliderTickRate)
                assertEquals(3238, map.sounds.limit())
                assertEquals(1, map.timingPoints.size)
                assertEquals(1740, map.difficultyPoints.size)
                assertEquals(1762, map.effectPoints.size)
                assertEquals(0.7F, map.stackLeniency)
                assertEquals(0, map.breaks.size)
            }
        }
    }

    @Test
    fun create() {
        assertBeatmap(GameMode.Osu, Beatmap(path = "./maps/2785319.osu"))
        assertBeatmap(GameMode.Osu, Beatmap(bytes = File("./maps/2785319.osu").readBytes()))
        assertBeatmap(GameMode.Taiko, Beatmap(path = "./maps/1028484.osu"))
        assertBeatmap(GameMode.Taiko, Beatmap(bytes = File("./maps/1028484.osu").readBytes()))
        assertBeatmap(GameMode.Catch, Beatmap(path = "./maps/2118524.osu"))
        assertBeatmap(GameMode.Catch, Beatmap(bytes = File("./maps/2118524.osu").readBytes()))
        assertBeatmap(GameMode.Mania, Beatmap(path = "./maps/1974394.osu"))
        assertBeatmap(GameMode.Mania, Beatmap(bytes = File("./maps/1974394.osu").readBytes()))
    }

    @Test
    fun clone() {
        val map = Beatmap(path = "./maps/2785319.osu")
        val clone = map.clone()
        assertNotEquals(map.ptr, clone.ptr)
        assertEquals(map.toString(), clone.toString())
        assertBeatmap(GameMode.Osu, clone)
    }
}