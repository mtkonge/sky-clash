#include "wifi.h"
#include "rfid_scanner.h"
#include <wiring_private.h>

#define IRQ_PIN 6
#define RSTO_PIN 7

#define OTHER_IRQ_PIN 4
#define OTHER_RSTO_PIN 5

#define OTHER_SDA 0
#define OTHER_SCL 1

TwoWire otherWire(&sercom3, 0, 1);   // Create the new wire instance assigning it to pin 0 and 1

RfidScanner rfid_scanner(RfidI2C(IRQ_PIN, RSTO_PIN, &Wire));
RfidScanner other_rfid_scanner(RfidI2C(OTHER_IRQ_PIN, OTHER_RSTO_PIN, &otherWire));

Wifi wifi(IPAddress(192, 168, 132, 183), 8080);

void setup() {
  Serial.begin(9600);

  pinPeripheral(OTHER_SDA, PIO_SERCOM);
  pinPeripheral(OTHER_SCL, PIO_SERCOM);
  otherWire.begin(2);

  while (!Serial) {
    delay(100);
  }

  wifi.connect();
  delay(500);
  wifi.ping();
  delay(1000);

  rfid_scanner.begin();
  other_rfid_scanner.begin();
}

extern "C" {
  void SERCOM3_Handler(void);
  void SERCOM3_Handler(void) {
    otherWire.onService();
  }
}


String response;
uint32_t last_hero_1_rfid = -1;
uint32_t last_hero_2_rfid = -1;

String format_rfid(String key, uint32_t rfid) {
  String result = key;
  if (rfid == 0) {
    result += "null";
  } else {
    result += '"';
    result += String(rfid);
    result += '"';
  }
  return result;
}

void loop() {
  uint32_t hero_1_rfid = rfid_scanner.read(100);
  uint32_t hero_2_rfid = other_rfid_scanner.read(100);
  if (last_hero_1_rfid == hero_1_rfid && last_hero_2_rfid == hero_2_rfid) { 
    return;
  }
  last_hero_1_rfid = hero_1_rfid;
  last_hero_2_rfid = hero_2_rfid;

  Serial.println(hero_1_rfid);
  Serial.println(hero_2_rfid);
  String hero_1_data = format_rfid(String("\"hero_1_rfid\":"), hero_1_rfid);
  String hero_2_data = format_rfid(String("\"hero_2_rfid\":"), hero_2_rfid);
  String data = String("{") + hero_1_data + ',' + hero_2_data + '}';
  Serial.println(String("data: ") + data);
  response = wifi.post("/update_heroes_on_board", data);
  Serial.println(response);
}
