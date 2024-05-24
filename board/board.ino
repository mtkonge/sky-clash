#include "wifi.h"
#include "rfid_scanner.h"
#include <wiring_private.h>

RfidPins rfid1Pins = { /*sda=*/11, /*scl=*/12, /*irq=*/5, /*rsto*/4 };

TwoWire* rfid1Wire = &Wire;
RfidScanner rfid1(RfidI2C(rfid1Pins.irq, rfid1Pins.rsto, rfid1Wire), rfid1Pins);

RfidPins rfid2Pins = { /*sda=*/0, /*scl=*/1, /*irq=*/6, /*rsto*/7 };

TwoWire otherWire(&sercom3, rfid2Pins.sda, rfid2Pins.scl);
TwoWire* rfid2Wire = &otherWire;
RfidScanner rfid2(RfidI2C(rfid2Pins.irq, rfid2Pins.rsto, rfid2Wire), rfid2Pins);

Wifi wifi(IPAddress(65, 108, 91, 32), 8080);

struct LedPins { int red, green, blue; };

LedPins led1 = { /*red=*/8, /*green=*/2, /*blue=*/3 };
LedPins led2 = { /*red=*/10, /*green=*/A4, /*blue=*/A3 };

int switchPin = 13;

void setup() {
  Serial.begin(9600);

  pinPeripheral(rfid2Pins.sda, PIO_SERCOM);
  pinPeripheral(rfid2Pins.scl, PIO_SERCOM);

  pinMode(led1.red, OUTPUT);
  pinMode(led1.green, OUTPUT);
  pinMode(led1.blue, OUTPUT);
  pinMode(led2.red, OUTPUT);
  pinMode(led2.green, OUTPUT);
  pinMode(led2.blue, OUTPUT);

  pinMode(switchPin, INPUT);

  analogWrite(led1.red, 255);
  analogWrite(led1.green, 255);
  analogWrite(led1.blue, 255);
  analogWrite(led2.red, 255);
  analogWrite(led2.green, 255);
  analogWrite(led2.blue, 255);


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
  rfid1.begin();
  Serial.println("rfid 2 begin");
  rfid2.begin();
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

  if (digitalRead(switchPin) == HIGH) {
    analogWrite(led1.red, 255);
    analogWrite(led1.green, 255);
    analogWrite(led1.blue, 255);
    analogWrite(led2.red, 255);
    analogWrite(led2.green, 255);
    analogWrite(led2.blue, 255);
  } else {
    analogWrite(led1.red, 255);
    analogWrite(led1.green, 0);
    analogWrite(led1.blue, 0);
    analogWrite(led2.red, 255);
    analogWrite(led2.green, 0);
    analogWrite(led2.blue, 0);
  }

  uint32_t hero_1_rfid = rfid2.read(100);
  uint32_t hero_2_rfid = rfid1.read(100);
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




