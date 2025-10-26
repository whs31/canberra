import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Window

ApplicationWindow {
    visible: true
    width: 640
    height: 480
    title: "Canberra"
    Material.theme: Material.Dark
    Text {
        anchors.centerIn: parent
        text: "hi from rust (i changed this line just now)"
    }

    Button {
        text: "Fuck"
    }
}