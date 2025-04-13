package xyz.theunknowngroup.app

import javafx.application.Application
import javafx.geometry.Insets
import javafx.scene.control.*
import javafx.scene.Scene
import javafx.scene.layout.VBox
import javafx.stage.Stage
import java.io.File
import java.net.URL
import java.util.concurrent.Executors

class App : Application() {

    private val versions = mutableListOf("1.20.4", "1.19")
    private val versionDropdown = ComboBox<String>()
    private val logArea = TextArea()

    override fun start(stage: Stage) {
        versionDropdown.items.addAll(versions)
        versionDropdown.selectionModel.selectFirst()

//        val downloadBtn = Button("Download")
//        val launchBtn = Button("Launch")
//
//        downloadBtn.setOnAction { downloadSel() }
//        launchBtn.setOnAction { launchM() }

        logArea.isEditable = false
        logArea.prefRowCount = 10

        val layout = VBox(10.0, Label("Select Version:"), versionDropdown, logArea)
        layout.padding = Insets(20.0)

        val scene = Scene(layout, 950.0, 530.0)
        scene.stylesheets.add(javaClass.getResource("/style.css").toExternalForm())

        stage.title = "UKMCL"
        stage.scene = scene
        stage.scene.stylesheets.add(javaClass.getResource("/style.css")!!.toExternalForm())
        stage.show()
    }
//
//    private fun downloadSel() {
//        val version = versionDropdown.value
//        val url = "https://piston-data.mojang.com/v1/objects/c0898ec7c6a5a2eaa317770203a1554260699994/client.jar"
//        val out = File("$version.jar")
//
//        log("Downloading $version..")
//        Executors.newSingleThreadExecutor().execute {
//            try {
//                URL(url).openStream().use { input ->
//                    out.outputStream().use { output ->
//                        input.copyTo(output)
//                    }
//                }
//                log("$version downloaded successfully.")
//            } catch (e: Exception) {
//                log("Download failed: ${e.message}")
//            }
//        }
//    }
//
//    private fun launchM() {
//        val version = versionDropdown.value
//        val jarFile = File("$version.jar")
//
//        if(!jarFile.exists()) {
//            log("JAR File not found: $version.jar")
//            return
//        }
//
//        try {
//            val process = ProcessBuilder(
//                "java", "-Xms1024m", "-Xmx2048m", "-cp",
//                "%APPDATA%\\.minecraft\\bin\\**\\*.jar",
//                "-Djava.library.path='%APPDATA%\\.minecraft\\bin\\natives'")
//                .inheritIO()
//                .start()
//            log("Launching")
//        } catch (e: Exception) {
//            log("Launch failed: ${e.message}")
//        }
//    }

    private fun log(message: String) {
        logArea.appendText("$message\n")
    }
}