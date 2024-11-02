.PHONY: all install uninstall clean

all: output/myscript

output/myscript: main.py settings.py
	@echo "Converting main.py..."
	@python -m nuitka --onefile --output-dir=output main.py
	@echo "Converting settings.py..."
	@python -m nuitka --onefile --output-dir=output settings.py
	@echo "completed in output/main.bin, output/payload.bin"

install: output/myscript
	@echo "Installing executable..."
	@cp output/myscript /usr/local/bin/myscript

uninstall:
	@echo "Uninstalling myscript..."
	@rm -f /usr/local/bin/myscript

clean:
	@echo "Cleaning up..."
	@rm -rf build dist output __pycache__ main.spec settings.spec