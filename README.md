# Ratatui Terminal Clock

This is a simple terminal-based clock application built using Rust and the `ratatui` crate. It displays the current time in a large, digital style within your terminal.

## Features

*   **Real-time Clock:** Displays the current time (hours, minutes, and seconds) and updates every half second.
*   **Large, Digital Style:** The clock digits are rendered with a blocky, segmented look to mimic a classic digital clock.
*   **Terminal UI:** Built using `ratatui` to render in the terminal, providing a clean and efficient UI.
*   **Keyboard Exit:** You can exit the application by pressing 'q', 'Esc' or 'Ctrl+c'.

## How It Works

The application uses the following core components:

1.  **Terminal Setup:**
    *   Enables raw mode for the terminal to capture key presses.
    *   Sets up an alternate screen to avoid messing up your shell.
    *   Initializes the `ratatui` terminal.

2.  **Main Loop:**
    *   Continuously draws the clock onto the terminal screen.
    *   Polls for keyboard input every 500 milliseconds.
    *   Exits the loop if the 'q', 'Esc' or 'Ctrl+c' is pressed.

3.  **Time Formatting:**
    *   Uses the `chrono` crate to get the current local time.
    *   The `format_time` function converts time into a `Text` object which handles the style and layout using  the function  `format_large_digit` to transform each digit of the time into a set of `Line` to display blocky digits with straight lines.

4.  **Digit Styling:**
    *   The `format_large_digit` function uses the characters "█" and spaces to create a large segmented digital style for each digit.
    *   Each number is made of a pre-defined blocky representation that is 8 character wide and 7 lines height.

5.  **Rendering:**
    *   The `ratatui` terminal is used to render the formatted time text in the center of the screen.
    *   The layout is divided into 3 vertical sections, and the text is rendered into the middle one.

6. **Terminal Restoration:**
   *  Disables the raw mode and exits the alternate screen when the application exits.

## Dependencies

*   `crossterm` for terminal manipulation.
*   `ratatui` for the terminal user interface.
*   `chrono` for time handling.

## How to Run

1.  Make sure you have Rust installed.
2.  Clone the repository, or create a new project and copy the content of the code.
3.  Add the dependencies to `Cargo.toml`:
    ```toml
    [dependencies]
    crossterm = "0.27"
    ratatui = "0.26"
    chrono = "0.4"
    ```
4.  Run the application:

    ```bash
    cargo run
    ```

## Code Structure
*  `main.rs`: Contains the main logic of the terminal clock.

## Further Improvements

*   Add colors.
*   Add a configuration option to the style of the clock.

## License

This project is open-source and available under the [MIT License](LICENSE).
