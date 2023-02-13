package xyz.cssxsh.rosu

public sealed class DifficultyAttributes(public val mode: GameMode) : Cloneable {
    internal abstract val ptr: Long

    public abstract fun stars(): Double

    public abstract fun maxCombo(): Long
}
