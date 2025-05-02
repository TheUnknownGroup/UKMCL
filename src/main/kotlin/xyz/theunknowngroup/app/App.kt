package xyz.theunknowngroup.app

import javafx.application.Application
import javafx.scene.control.TextArea
import javafx.stage.Stage
import xyz.theunknowngroup.app.scenes.startPage
import xyz.theunknowngroup.images.Images.S.icon

var w = 950.0
var h = 530.0
class App : Application() {
    override fun start(stage: Stage) {
        stage.apply {
            title = "UKMCL"
            scene = startPage(stage)
            isResizable = false
            try {
                icons.add(icon)
            } catch (e: Exception) {
                error("Icon failed to load: ${e.message}")
            }
        }
        stage.show()
    }
}

fun log(message: String, logArea: TextArea) {
    logArea.appendText("$message\n")
}