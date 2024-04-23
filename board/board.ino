#include <Adafruit_PN532.h>


Adafruit_PN532 rfid(0, 1, &Wire);

void setup() {
  Serial.begin(9600);

  while (!Serial) {
    delay(100);
  }

  Serial.println("Start");

  bool success = rfid.begin();
  if (!success) {
    Serial.println("RFID failed");
  }

  uint32_t version = rfid.getFirmwareVersion();
  Serial.println(String("version = ") + version);
}
 
void loop() {
  uint8_t success;
  uint8_t uid[] = { 0, 0, 0, 0, 0, 0, 0 };
  uint8_t uidLength;

  success = rfid.readPassiveTargetID(PN532_MIFARE_ISO14443A, uid, &uidLength);

  if (success) {
    Serial.println("Found an ISO14443A card");
    Serial.print("  UID Length: ");
    Serial.print(uidLength, DEC);Serial.println(" bytes");
    Serial.print("  UID Value: ");
  } else {
    Serial.println("Failed");
    delay(500);
  }
}
