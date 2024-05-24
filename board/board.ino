#include "wifi.h"
#include "rfid_scanner.h"
#include <wiring_private.h>

#define RFID_2_IRQ 6
#define RFID_2_RSTO 7

#define RFID_2_SDA 1
#define RFID_2_SCL 0

#define RFID_1_IRQ 5
#define RFID_1_RSTO 4

#define RFID_1_SDA 11
#define RFID_1_SCL 12

RfidPins rfid_1_pins = {
  RFID_1_SDA,
  RFID_1_SCL,
  RFID_1_IRQ,
  RFID_1_RSTO,
};

TwoWire* rfid_1_wire = &Wire;
RfidScanner rfid_1(RfidI2C(rfid_1_pins.irq, rfid_1_pins.rsto, rfid_1_wire), rfid_1_pins);

RfidPins rfid_2_pins = {
  RFID_2_SDA,
  RFID_2_SCL,
  RFID_2_IRQ,
  RFID_2_RSTO,
};

TwoWire otherWire(&sercom3, rfid_2_pins.sda, rfid_2_pins.scl);
TwoWire* rfid_2_wire = &otherWire;
RfidScanner rfid_2(RfidI2C(rfid_2_pins.irq, rfid_2_pins.rsto, rfid_2_wire), rfid_2_pins);

Wifi wifi(IPAddress(65, 108, 91, 32), 8080);

void setup() {
  Serial.begin(9600);

  pinPeripheral(RFID_2_SDA, PIO_SERCOM);
  pinPeripheral(RFID_2_SCL, PIO_SERCOM);

  otherWire.begin(2);
  Serial.println(availableMemory());
  while (!Serial) {
    delay(100);
  }

  // wifi.connect();
  // delay(500);
  // wifi.ping();
  // delay(1000);

  Serial.println("rfid 1 begin");
  rfid_1.begin();
  Serial.println("rfid 2 begin");
  rfid_2.begin();
  Serial.println("rfid loaded");
}

extern "C" {
  void SERCOM3_Handler(void);
  void SERCOM3_Handler(void) {
    otherWire.onService();
  }
}


int availableMemory() {
    // Use 1024 with ATmega168
    int size = 2048;
    byte *buf;
    while ((buf = (byte *) malloc(--size)) == NULL);
        free(buf);
    return size;
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
  uint32_t hero_1_rfid = rfid_2.read(100);
  uint32_t hero_2_rfid = rfid_1.read(100);
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
  // response = wifi.post("/update_heroes_on_board", data);
  Serial.println(response);
}
