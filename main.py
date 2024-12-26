import sys
from PyQt6 import *
from PyQt6.QtWidgets import *
from PyQt6.QtGui import *
from PyQt6.QtCore import *
from PyQt6 import QtGui

def main():
    app = QApplication(sys.argv)

    window = QMainWindow()
    window.setWindowTitle("UKMCL")
    window.setWindowIcon(QtGui.QIcon("./assets/32x32.png"))
    window.setStyleSheet("background-image: url(./assets/background.png)")
    window.setFixedSize(950, 530)

    label = QLabel("Welcome! :wave:", window)
    label.move(100, 100)
    label.setStyleSheet("background: none")
    label.setAlignment(Qt.AlignmentFlag.AlignHCenter)
    label.setFont()

    new = QLabel("", window)

    window.pixmap = QPixmap("./assets/icon.png")
    new.setPixmap(window.pixmap)

    new.move(15, 15)
    new.resize(32, 32)

    new.setStyleSheet("background: none")

    lab = QLabel("", None)

    lab.setOpenExternalLinks(True)

    window.show()

    sys.exit(app.exec())

if __name__ == "__main__":
    main()