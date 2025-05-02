package xyz.theunknowngroup.app.scenes

import javafx.geometry.Pos
import javafx.scene.Scene
import javafx.scene.control.Label
import javafx.scene.control.TextField
import javafx.scene.layout.GridPane
import javafx.scene.layout.Priority
import javafx.scene.layout.VBox
import javafx.stage.Stage
import xyz.theunknowngroup.app.h
import xyz.theunknowngroup.app.w
import xyz.theunknowngroup.images.Images.S.back

fun signUp(stage: Stage): Scene {
    // Components
    val user1 = Label("Username:").apply { id = "user1" }
    val usern1 = TextField().apply { id = "usern1" }
    val pass1 = Label("Password:").apply { id = "pass1" }
    val passw1 = TextField().apply { id = "passw1" }
    passw1.setOnKeyPressed {
        event -> if (event.code == key) {
            println(usern1.text)
            println(passw1.text)
            reset(usern1, passw1)
        }
    }

    // Layout
    val lay2 = GridPane().apply {
        add(user1, 0, 0)
        add(usern1, 0, 1)

        add(pass1, 0, 2)
        add(passw1, 0, 3)

        // config
        id = "lay2"
        maxWidth = 350.0
        maxHeight = 350.0
        alignment = Pos.CENTER
    }

    GridPane.setHgrow(lay2, Priority.SOMETIMES)

    val layout1 = VBox(10.0).apply {
        children.addAll(lay2)
        style = "-fx-background-image: url('$back');"
        id = "layout1"
        alignment = Pos.CENTER_LEFT
        VBox.setVgrow(lay2, Priority.SOMETIMES)
    }

    val scene = Scene(layout1, w, h).apply {
        stylesheets.add(javaClass.getResource("/signup.css")!!.toExternalForm())
    }

    stage.scene = scene

    return scene
}