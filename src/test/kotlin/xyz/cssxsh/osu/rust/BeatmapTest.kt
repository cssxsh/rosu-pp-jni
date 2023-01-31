package xyz.cssxsh.osu.rust

import java.io.File
import kotlin.test.*

internal class BeatmapTest {

    private fun assertBeatmap(expected: GameMode, map: Beatmap) {
        when (expected) {
            GameMode.Osu -> {
                assertEquals(GameMode.Osu, map.mode)
                assertEquals(14, map.version)
                assertEquals(9.3F, map.ar)
                assertEquals(8.8F, map.od)
                assertEquals(4.5F, map.cs)
                assertEquals(5.0F, map.hp)
            }
            GameMode.Taiko -> {
                assertEquals(GameMode.Taiko, map.mode)
                assertEquals(14, map.version)
                assertEquals(8.0F, map.ar)
                assertEquals(5.0F, map.od)
                assertEquals(2.0F, map.cs)
                assertEquals(6.0F, map.hp)
            }
            GameMode.Catch -> {
                assertEquals(GameMode.Catch, map.mode)
                assertEquals(14, map.version)
                assertEquals(8.0F, map.ar)
                assertEquals(8.0F, map.od)
                assertEquals(3.5F, map.cs)
                assertEquals(5.0F, map.hp)
            }
            GameMode.Mania -> {
                assertEquals(GameMode.Mania, map.mode)
                assertEquals(14, map.version)
                assertEquals(5.0F, map.ar)
                assertEquals(8.0F, map.od)
                assertEquals(4.0F, map.cs)
                assertEquals(9.0F, map.hp)
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
}