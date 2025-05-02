package xyz.theunknowngroup.images

import javafx.scene.image.Image

class Images {
    object S {
        val icon = Image(javaClass.getResource("/assets/icon.png")!!.toExternalForm())
        val back: String = javaClass.getResource("/assets/background.png")!!.toExternalForm()
    }
}