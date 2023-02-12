package xyz.cssxsh.rosu

import java.io.*
import java.nio.file.*
import java.util.concurrent.atomic.*

/**
 * ROSU-PP JNI Loader
 */
public object Library {
    @PublishedApi
    internal const val ROSU_LIBRARY_PATH_PROPERTY: String = "rosu.library.path"
    @PublishedApi
    internal var loaded: AtomicBoolean = AtomicBoolean(false)
    @PublishedApi
    internal val cacheRoot: String by lazy {
        "${System.getProperty("user.home")}/.rosu/"
    }
    @PublishedApi
    internal val hostOs: String by lazy {
        val osName = System.getProperty("os.name")
        when {
            osName == "Mac OS X" -> "macos"
            osName.startsWith("Win") -> "windows"
            "The Android Project" == System.getProperty("java.specification.vendor") -> "android"
            osName == "Linux" -> "linux"
            else -> throw RuntimeException("Unknown OS $osName")
        }
    }
    @PublishedApi
    internal val hostArch: String by lazy {
        when (val osArch = System.getProperty("os.arch")) {
            "x86_64", "amd64" -> "x64"
            "aarch64" -> "arm64"
            else -> throw RuntimeException("Unknown arch $osArch")
        }
    }

    private val libraryPath = System.getProperty(ROSU_LIBRARY_PATH_PROPERTY)
    private var copyDir: File? = null

    /**
     * 尝试加载 ROSU_LIBRARY
     */
    public fun staticLoad() {
        if (loaded.compareAndSet(false, true)) {
            load()
        }
    }

    private fun loadLibraryOrCopy(library: File) {
        try {
            System.load(library.absolutePath)
        } catch (e: UnsatisfiedLinkError) {
            if (e.message?.contains("already loaded in another classloader") == true) {
                copyDir = Files.createTempDirectory("rosu").toFile()
                val tempFile = copyDir!!.resolve(library.name)
                Files.copy(library.toPath(), tempFile.toPath(), StandardCopyOption.REPLACE_EXISTING)
                tempFile.deleteOnExit()
                System.load(tempFile.absolutePath)
            } else {
                throw e
            }
        }
    }

    private fun unpackIfNeeded(dest: File, resourceName: String): File {
        val file = File(dest, resourceName)
        if (!file.exists()) {
            val tempFile = File.createTempFile("rosu", "", dest)
            Library::class.java.getResourceAsStream("/$resourceName")!!.use { input ->
                Files.copy(input, tempFile.toPath(), StandardCopyOption.REPLACE_EXISTING)
            }
            Files.move(tempFile.toPath(), file.toPath(), StandardCopyOption.ATOMIC_MOVE)
        }
        return file
    }

    @Synchronized
    internal fun load() {
        val name = "rosu-$hostOs-$hostArch"
        val platformName = System.mapLibraryName(name)

        if (hostOs == "android") {
            System.loadLibrary(name)
            return
        }

        // First try: system property is set.
        if (libraryPath != null) {
            val library = File(libraryPath, platformName)
            loadLibraryOrCopy(library)
            return
        }

        val jvmFiles = File(System.getProperty("java.home"), if (hostOs == "windows") "bin" else "lib")
        val pathInJvm = jvmFiles.resolve(platformName)
        if (pathInJvm.exists()) {
            loadLibraryOrCopy(pathInJvm)
            return
        }

        val hashResourceStream = Library::class.java.getResourceAsStream(
            "/$platformName.sha256"
        ) ?: throw RuntimeException(
            "Cannot find $platformName.sha256, proper native dependency missing."
        )
        val hash = hashResourceStream.use { it.bufferedReader().readLine() }

        val cacheDir = File(cacheRoot, hash)
        cacheDir.mkdirs()
        val library = unpackIfNeeded(cacheDir, platformName)
        loadLibraryOrCopy(library)
    }
}