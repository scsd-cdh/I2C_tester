/* Serial Monitor Example, Teensyduino Tutorial #3
   http://www.pjrc.com/teensy/tutorial3.html

   After uploading this to your board, use Serial Monitor
   to view the message.  When Serial is selected from the
   Tools > USB Type menu, the correct serial port must be
   selected from the Tools > Serial Port AFTER Teensy is
   running this code.  Teensy only becomes a serial device
   while this code is running!  For non-Serial types,
   the Serial port is emulated, so no port needs to be
   selected.

   This example code is in the public domain.
*/

void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, HIGH);
  Serial.begin(9600);

  // Wait until the Serial Monitor or connection is open
  while (!Serial) {
    // Wait until the serial connection is established
    delay(10);
  }

  Serial.println("Waiting for message...");
}

void loop() {
  // Serial.println("message");
  //  Check if data is available to read
  if (Serial.available() > 0) {
    // Read the incoming message
    String message = Serial.readStringUntil('\n');
    // Send the message back
    if (message) {
      Serial.print("Echoing: ");
      Serial.println(message);
    }
  }
}
