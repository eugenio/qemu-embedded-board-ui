# qemu-embedded-board-ui

This project aims to create a GUI to visualize the graphic representation of any generic board emulable with QEMU with its own peripherals and visualize their emulated output with graphical widgets. This is going to be done by leveraging the QEMU capability of exchanging JSON messages via the qemu-qmp protocol.
The main language of this project will be Rust.
The gui library is Iced: https://github.com/hecrj/iced
The QEMU communication library is: https://github.com/arcnmx/qapi-rs


## State of the project

Atm the state of the project is just a mock up  with a single led (represented by a series .svg file, one for each colour and the off state) that is "turned on or off" by pressing either of button on or off buttons.

## Roadmap
The roadmap has these milestones:
1. Obtain the representation of a generic schematic from kicad( in the beginning , in the future also from other schematic editors ) and translating it into a graphical representation of its basic components, peripheral and connections.

2. Connect the elements of the gui to qemu output through the qapi-rs library.
