#include "wifi.h"
#include "rfid_scanner.h"

#define IRQ_PIN 1
#define RSTO_PIN 0



RfidScanner rfid_scanner(IRQ_PIN, RSTO_PIN);

Wifi wifi(IPAddress(192, 168, 132, 183), 8080);

void setup() {
  Serial.begin(9600);

  while (!Serial) {
    delay(100);
  }

  wifi.connect();
  delay(500);
  wifi.ping();
  delay(1000);
  String response = wifi.post("/create_hero", "{\"rfid\": \"1234\", \"hero_type\": 0}");
  Serial.println(response);

  rfid_scanner.begin();
}


void loop() {
  rfid_scanner.read();
}
