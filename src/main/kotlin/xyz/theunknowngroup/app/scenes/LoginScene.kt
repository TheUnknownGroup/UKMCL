package xyz.theunknowngroup.app.scenes

import javafx.geometry.Pos
import javafx.scene.Scene
import javafx.scene.control.Label
import javafx.scene.control.TextField
import javafx.scene.input.KeyCode
import javafx.scene.layout.GridPane
import javafx.scene.layout.Priority
import javafx.scene.layout.VBox
import javafx.stage.Stage
import xyz.theunknowngroup.app.h
import xyz.theunknowngroup.app.w
import xyz.theunknowngroup.images.Images.S.back

var key = KeyCode.ENTER

fun logIn(stage: Stage): Scene {
    // Components
    val user = Label("Username:").apply { id = "user" }
    val usern = TextField().apply { id = "usern" }
    val pass = Label("Password:").apply { id = "pass" }
    val passw = TextField().apply { id = "passw" }
    passw.setOnKeyPressed {
        event -> if (event.code == key) {
            println(usern.text)
            println(passw.text)
            reset(usern, passw)
        }
    }

    // Layout
    val lay1 = GridPane().apply {
        add(user, 0, 0)
        add(usern, 0, 1)

        add(pass, 0, 2)
        add(passw, 0, 3)

        // config
        id = "lay1"
        maxWidth = 350.0
        maxHeight = 350.0
        alignment = Pos.CENTER
    }

    GridPane.setHgrow(lay1, Priority.SOMETIMES)

    val layout = VBox(10.0).apply {
        children.addAll(lay1)
        style = "-fx-background-image: url('$back');"
        id = "layout"
        alignment = Pos.CENTER_LEFT
        VBox.setVgrow(lay1, Priority.SOMETIMES)
    }

    val scene = Scene(layout, w, h).apply {
        stylesheets.add(javaClass.getResource("/login.css")!!.toExternalForm())
    }

    stage.scene = scene

    return scene
}
fun reset(text: TextField, text2: TextField) {
    text.text = ""
    text2.text = ""
}