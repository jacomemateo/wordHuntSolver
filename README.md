# Raspberry Pi Bluetooth Mouse Emulator for Word Hunt Solving (WHS)

This some nerd shit I swear i get bitches... 😭😭😭😭

## Description:
The Raspberry Pi Bluetooth Mouse Emulator for Word Hunt Solving (WHS) project aims to develop a system utilizing a Raspberry Pi to emulate a mouse pointer. This system will connect to an iPhone via Bluetooth using the accessibility touch option in settings. The implementation will be done primarily in the Rust programming language. The ultimate goal is to create a program capable of solving word hunts in iMessage games, specifically the Word Hunt game.

## Resources:
- [EmuBTHID](https://github.com/Alkaid-Benetnash/EmuBTHID)
- [zbus crate](https://crates.io/crates/zbus)
- [D-Bus Documentation](https://www.freedesktop.org/wiki/Software/dbus/#index1h1)
- [Keyboard and Mouse Emulation on Raspberry Pi](https://github.com/thanhlev/keyboard_mouse_emulate_on_raspberry)
- Linux Bluetooth Low Energy (BLE) documentation

## Key Learning Objectives:
1. Gain proficiency in utilizing D-Bus for inter-process communication.
2. Master the integration of Bluez with D-Bus for Bluetooth communication.
3. Transition programming skills from Python to Rust.
4. Develop expertise in low-level system programming concepts relevant to Bluetooth communication and mouse emulation.

## Project Plan:

### Part A - Mouse Emulator Development:
1. Create a Bluetooth server on the Raspberry Pi:
    - Implement discoverability functionality.
    - Enable pairing with the iPhone.
2. Emulate a mouse pointer:
    - Investigate methods for obtaining the position of the phone screen. Determine if this information can be retrieved through the Bluetooth API or if additional methods are required.
3. Develop a mouse control abstraction layer:
    - Design a class to facilitate mouse movement using function calls.
    - Determine whether absolute mouse position or other methods are appropriate.
    - Research and implement mouse acceleration mechanisms.

### Part B - Word Hunt Solver Implementation:
1. Utilize the mouse control abstraction layer created in Part A to control the mouse pointer.
2. Determine the starting coordinates of the Word Hunt grid:
    - Experiment to identify the initial coordinates of the hunt grid and establish movement constants.
3. Implement a word hunt solving algorithm:
    ```
    |  A  |  B  |  C  |  D  |
	|  E  |  F  |  G  |  H  |
	|  I  |  J  |  K  |  L  |
	|  M  |  N  |  O  |  P  |
    ```
    - Convert the hunt grid into a string format (e.g., "ABCDEFGHIJKLMNOP").
    - Load the string into a 2D vector.
    - Load a list of English words from a dictionary.
    - Implement a brute-force algorithm to find valid words by checking adjacent letter combinations against the English word list.
    - Store the positions of matched words in a suitable data structure.
4. Use the mouse control class from Part A to move the mouse pointer to select identified words within the hunt grid.

## Additional Considerations:
- Implement robust error handling mechanisms to manage potential failures during Bluetooth communication and mouse emulation.
- Thoroughly test the system to ensure reliability and accuracy, especially in scenarios with varying screen resolutions.
- Document code and design decisions comprehensively for future reference and maintenance purposes.
- Optimize code performance as necessary, particularly in the word hunt solving algorithm, to enhance efficiency.


