package xyz.theunknowngroup.app.scenes

import javafx.scene.Scene
import javafx.scene.layout.VBox
import javafx.stage.Stage
import xyz.theunknowngroup.app.App.Companion.h
import xyz.theunknowngroup.app.App.Companion.w

fun launchScreen(stage: Stage): Scene {

    val layout = VBox(10.0).apply {

    }

    val scene = Scene(layout, w, h).apply {
        stylesheets.add(javaClass.getResource("/login.css")!!.toExternalForm())
    }

    stage.scene = scene

    return scene
}