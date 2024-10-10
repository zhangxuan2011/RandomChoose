import sys
import json
from qtpy.QtWidgets import QApplication, QMainWindow, QMessageBox, QPushButton
from qtpy import uic


app = QApplication(sys.argv)
ui = uic.loadUi("./ui/settings.ui")
ui.show()

applyButton: QPushButton = ui.applyButton

def applyopt():
    try:
        minNumValue = ui.minNum.value()
        maxNumValue = ui.maxNum.value()
        chooseMode = "listDel"
        basedata = {'minNum': minNumValue, 'maxNum': maxNumValue}
        optionaldata = {'blacklist': [], 'chooseMode': chooseMode}
        data = {"basic": basedata, "optional": optionaldata}
        with open("config.json", "w") as f:
            json.dump(data, f, indent=4)
        QMessageBox.information(None, "提示", "写入成功!")
    except Exception as e:
        QMessageBox.critical(None, "错误", f"在试图将配置写入config.json时发生了一些问题\n错误代码:{e}")

applyButton.clicked.connect(applyopt)

sys.exit(app.exec())