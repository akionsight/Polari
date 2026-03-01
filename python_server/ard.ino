#include <IRremote.hpp>
#include <LiquidCrystal.h>

LiquidCrystal lcd(12, 11, 5, 4, 3, 2);

const int RECV_PIN = 7;
String data = "";  // Moved outside loop so it persists between button presses

void setup() {
  Serial.begin(9600);
  IrReceiver.begin(RECV_PIN, ENABLE_LED_FEEDBACK);

  lcd.begin(16, 2);
  lcd.setCursor(0, 0);
  lcd.print("Enter number:");
  lcd.setCursor(0, 1);
  lcd.print("                "); // Clear row 2
}

void updateDisplay() {
  lcd.setCursor(0, 1);
  lcd.print("                "); // Clear old content
  lcd.setCursor(0, 1);
  lcd.print(data);
}
void scrollText(String msg, int row, int delayMs) {
  // Pad with spaces so text slides in and out cleanly
  msg = "                " + msg + "                ";

  for (int i = 0; i < msg.length() - 15; i++) {
    lcd.setCursor(0, row);
    lcd.print(msg.substring(i, i + 16));
    delay(delayMs);
  }
}

void loop() {

    if (Serial.available()) {
    String msg = Serial.readStringUntil('\n');
    // msg.trim();

    lcd.setCursor(0, 0);
    lcd.print("                ");  // Clear row 1
    lcd.setCursor(0, 0);
    // lcd.print('here');
    scrollText(msg, 0, 225)   ;             // Print serial message on row 1
  }


  if (IrReceiver.decode()) {

    switch (IrReceiver.decodedIRData.decodedRawData) {
      case 0xF30CFF00: data += "1"; break;
      case 0xE718FF00: data += "2"; break;
      case 0xA15EFF00: data += "3"; break;
      case 0xF708FF00: data += "4"; break;
      case 0xE31CFF00: data += "5"; break;
      case 0xA55AFF00: data += "6"; break;
      case 0xBD42FF00: data += "7"; break;
      case 0xAD52FF00: data += "8"; break;
      case 0xB54AFF00: data += "9"; break;
      case 0xE916FF00: data += "0"; break;

      case 0xBC43FF00:  // OK / Submit button
        Serial.println(data);
        lcd.setCursor(0, 0);
        lcd.print("Submitted:      ");
        updateDisplay();
        delay(2000);           // Show result for 2 seconds
        data = "";             // Clear after submit
        lcd.setCursor(0, 0);
        lcd.print("Enter number:   ");
        updateDisplay();
        break;
    }

    // Show live input as user types
    updateDisplay();
    // Serial.println("Current: " + data);

    IrReceiver.resume();
  }
}
