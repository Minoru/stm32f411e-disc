stm32f411e-disc
===============

_stm32f411e-disc_ contains a board support package for the [STM32F411E-DISC][]
microcontroller board, enabling you to write firmware using the Rust language.
That experimentation board features four user-programmable LEDs, a push-button,
a gyroscope, an accelerometer, a magnetic sensor, a thermometer, a microphone,
an audio DAC with a 3.5mm jack connector, a Full-Speed USB OTG micro-B
connector, complete with 128KB of RAM and 512 KB of Flash storage. It also
contains a (non-removable) capable ST-Link V2 debugging interface.

This crate is a work in progress. Current support status for peripherals is:

- [x] LEDs
- [x] push-buttons
- [x] gyroscope
- [x] accelerometer
- [x] magnetic sensor
- [x] thermometer
- [ ] microphone
- [ ] audio DAC
- [ ] USB OTG

Contributions of all kinds are very much welcome!

[STM32F411E-DISC]: https://www.st.com/en/evaluation-tools/32f411ediscovery.html

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
