package xyz.cssxsh.rosu

import kotlin.test.*

internal class ScoreStateTest {

    @Test
    fun create() {
        val state = ScoreState()

        assertEquals(0, state.maxCombo)
        state.maxCombo = 909
        assertEquals(909, state.maxCombo)

        assertEquals(0, state.geki)
        state.geki = 10
        assertEquals(10, state.geki)

        assertEquals(0, state.katu)
        state.katu = 100
        assertEquals(100, state.katu)

        assertEquals(0, state.n300)
        state.n300 = 100
        assertEquals(100, state.n300)

        assertEquals(0, state.n100)
        state.n100 = 100
        assertEquals(100, state.n100)

        assertEquals(0, state.n50)
        state.n50 = 100
        assertEquals(100, state.n50)

        assertEquals(0, state.misses)
        state.misses = 100
        assertEquals(100, state.misses)

        assertEquals(400, state.totalHits(GameMode.Osu))
        assertEquals(300, state.totalHits(GameMode.Taiko))
        assertEquals(500, state.totalHits(GameMode.Catch))
        assertEquals(510, state.totalHits(GameMode.Mania))
    }
}