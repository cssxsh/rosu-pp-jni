import java.io.ByteArrayOutputStream

plugins {
    kotlin("jvm") version "1.7.22"

    id("me.him188.maven-central-publish") version "1.0.0-dev-3"
}

group = "xyz.cssxsh.osu"
version = "0.0.1"

mavenCentralPublish {
    useCentralS01()
    singleDevGithubProject("cssxsh", "rosu-pp-jni")
    licenseFromGitHubProject("AGPL-3.0")
    workingDir = System.getenv("PUBLICATION_TEMP")?.let { file(it).resolve(projectName) }
        ?: buildDir.resolve("publishing-tmp")
}

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
}

kotlin {
    explicitApi()
    target.compilations {
        all {
            kotlinOptions {
                jvmTarget = "1.8"
            }
        }
    }
}

java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}

tasks {
    test {
        useJUnitPlatform()
    }

    create("generateJniHeaders") {
        group = "other"
        dependsOn(getByName("compileKotlin"))

        // For caching
        val path = "build/generated/jni"
        inputs.dir("src/main/kotlin")
        outputs.dir(path)

        doLast {
            val javaHome = org.gradle.internal.jvm.Jvm.current().javaHome
            val javap = javaHome.resolve("bin").walk()
                .firstOrNull { it.name.startsWith("javap") }?.absolutePath ?: error("javap not found")
            val javac = javaHome.resolve("bin").walk()
                .firstOrNull { it.name.startsWith("javac") }?.absolutePath ?: error("javac not found")
            val classes = buildDir.resolve("classes/kotlin/main")
            val tmpDir = buildDir.resolve("tmp/jvmJni").apply { mkdirs() }

            val bodyExtractingRegex = """^.+\Rpublic \w* ?class ([^\s]+).*\{\R((?s:.+))\}\R$""".toRegex()
            val nativeMethodExtractingRegex = """.*\bnative\b.*""".toRegex()

            classes.walkTopDown()
                .filter { "META" !in it.absolutePath }
                .forEach { file ->
                    if (!file.isFile) return@forEach

                    val output = ByteArrayOutputStream().use {
                        project.exec {
                            commandLine(javap, "-private", "-cp", buildDir.absolutePath, file.absolutePath)
                            standardOutput = it
                        }.assertNormalExitValue()
                        it.toByteArray()
                    }.decodeToString()

                    val (qualifiedName, methodInfo) = bodyExtractingRegex.find(output)?.destructured ?: return@forEach

                    val lastDot = qualifiedName.lastIndexOf('.')
                    val packageName = qualifiedName.substring(0, lastDot)
                    val className = qualifiedName.substring(lastDot + 1, qualifiedName.length)

                    val nativeMethods = nativeMethodExtractingRegex.findAll(methodInfo)
                        .map { it.groups }
                        .flatMap { it.asSequence().mapNotNull { group -> group?.value } }
                        .toList()
                    if (nativeMethods.isEmpty()) return@forEach

                    val source = buildString {
                        appendLine("package $packageName;")
                        appendLine("public class $className {")
                        for (method in nativeMethods) {
                            if ("()" in method) appendLine(method)
                            else {
                                val updatedMethod = StringBuilder(method).apply {
                                    var count = 0
                                    var i = 0
                                    while (i < length) {
                                        if (this[i] == ',' || this[i] == ')') insert(
                                            i,
                                            " arg${count++}".also { i += it.length + 1 })
                                        else i++
                                    }
                                }
                                appendLine(updatedMethod)
                            }
                        }
                        appendLine("}")
                    }
                    val outputFile = tmpDir.resolve(packageName.replace(".", "/"))
                        .apply { mkdirs() }
                        .resolve("$className.java")
                        .apply { delete() }.apply { createNewFile() }
                    outputFile.writeText(source)

                    project.exec {
                        commandLine(javac, "-h", path, outputFile.absolutePath)
                    }.assertNormalExitValue()
                }
        }
    }
}