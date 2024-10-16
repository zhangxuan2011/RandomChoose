import sys
import json
from PyQt6.QtWidgets import QApplication, QMainWindow, QMessageBox, QPushButton
from PyQt6 import uic
import openpyxl


# 创建UI
app = QApplication(sys.argv)
ui = uic.loadUi("./ui/settings.ui")
ui.show()

# 设置Object
applyButton: QPushButton = ui.applyButton
blacklist = list()
nickname = list()
showMode = 'number'

# 打开 Excel 文件
try:
	workbook = openpyxl.load_workbook('nickname.xlsx')
	sheet = workbook.active
	for cell in sheet['B']:
		nickname.append(cell.value)
except:
	pass

def applyopt():
    try:
        minNumValue = ui.minNum.value()
        maxNumValue = ui.maxNum.value()
        chooseMode = "listDel"
        basedata = {'minNum': minNumValue, 'maxNum': maxNumValue, 'chooseMode': chooseMode}
        optionaldata = {'blacklist': blacklist, 'nickname': nickname, 'showMode': showMode}
        data = {"basic": basedata, "optional": optionaldata}
        with open("config.json", "w") as f:
            json.dump(data, f, indent=4)
        QMessageBox.information(None, "提示", "写入成功!")
    except Exception as e:
        QMessageBox.critical(None, "错误", f"在试图将配置写入config.json时发生了一些问题\n错误代码:{e}")

applyButton.clicked.connect(applyopt)

sys.exit(app.exec())
