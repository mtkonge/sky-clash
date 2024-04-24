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

  rfid_scanner.begin();
}




String response;
uint32_t last_rfid = -1;


void loop() {
  uint32_t read_rfid = rfid_scanner.read(100);
  if (last_rfid == read_rfid) { 
    return;
  }
  last_rfid = read_rfid;

  Serial.println(read_rfid);
  if (read_rfid != 0) {
    String data = String("{\"hero_1_rfid\": \"") + read_rfid + "\", \"hero_2_rfid\": null}";
    Serial.println(String("data: ") + data);
    response = wifi.post("/update_heroes_on_board", data);
  } else {
    String data = "{\"hero_1_rfid\": null, \"hero_2_rfid\": null}";
    Serial.println(String("data: ") + data);
    response = wifi.post("/update_heroes_on_board", data);
  }
  Serial.println(response);
}
